//! Data structure to store a list of userspace applications.

use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::{Deref, DerefMut};
use core::ptr::{write, NonNull};

use crate::callback::AppId;
use crate::process::{Error, ProcessType};
use crate::sched::Kernel;

pub struct Borrowed<'a, T: 'a + ?Sized> {
    data: &'a mut T,
    appid: AppId,
}

impl<'a, T: 'a + ?Sized> Borrowed<'a, T> {
    fn new(data: &'a mut T, appid: AppId) -> Borrowed<'a, T> {
        Borrowed {
            data: data,
            appid: appid,
        }
    }

    pub fn appid(&self) -> AppId {
        self.appid
    }
}

impl<'a, T: 'a + ?Sized> Deref for Borrowed<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: 'a + ?Sized> DerefMut for Borrowed<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

pub struct AppliedGrant<'a, T: 'a> {
    process: &'a dyn ProcessType,
    grant_num: usize,
    grant: &'a mut T,
    _phantom: PhantomData<T>,
}

impl<'a, T: Default> AppliedGrant<'a, T> {
    fn get_or_allocate(grant: &Grant<T>, process: &'a dyn ProcessType) -> Result<Self, Error> {
        // Here is an example of how the grants are laid out in a
        // process's memory:
        //
        // Mem. Addr.
        // 0x0040000  ┌────────────────────
        //            │   GrantPointer0 [0x003FFC8]
        //            │   GrantPointer1 [0x003FFC0]
        //            │   ...
        //            │   GrantPointerN [0x0000000 (NULL)]
        //            ├────────────────────
        //            │   Process Control Block
        // 0x003FFE0  ├────────────────────
        //            │   GrantRegion0
        // 0x003FFC8  ├────────────────────
        //            │   GrantRegion1
        // 0x003FFC0  ├────────────────────
        //            │
        //            │   --unallocated--
        //            │
        //            └────────────────────
        //
        // An array of pointers (one per possible grant region)
        // point to where the actual grant memory is allocated
        // inside of the process. The grant memory is not allocated
        // until the actual grant region is actually used.
        //
        // This function provides the app access to the specific
        // grant memory, and allocates the grant region in the
        // process memory if needed.

        // Get the GrantPointer to start. Since process.rs does not know
        // anything about the datatype of the grant, and the grant
        // memory may not yet be allocated, it can only return a `*mut
        // u8` here. We will eventually convert this to a `*mut T`.
        let grant_num = grant.grant_num;
        let appid = process.appid();
        if let Some(untyped_grant_ptr) = process.get_grant_ptr(grant_num) {
            // If the grant pointer is NULL then the memory for the
            // GrantRegion needs to be allocated. Otherwise, we can
            // convert the pointer to a `*mut T` because we know we
            // previously allocated enough memory for type T.
            unsafe {
                let typed_grant_pointer = if untyped_grant_ptr.is_null() {
                    // Allocate space in the process's memory for
                    // something of type `T` for the grant.
                    //
                    // Note: This allocation is intentionally never
                    // freed.  A grant region is valid once allocated
                    // for the lifetime of the process.
                    let alloc_size = size_of::<T>();
                    let new_region = appid
                        .kernel
                        .process_map_or(Err(Error::NoSuchApp), appid, |process| {
                            process
                                .alloc(alloc_size, align_of::<T>())
                                .map_or(Err(Error::OutOfMemory), |buf| {
                                    // Convert untyped `*mut u8` allocation to allocated type
                                    let ptr = NonNull::cast::<T>(buf);

                                    Ok(ptr)
                                })
                        })?;

                    // We use `ptr::write` to avoid `Drop`ping the uninitialized memory in
                    // case `T` implements the `Drop` trait.
                    write(new_region.as_ptr(), T::default());

                    // Update the grant pointer in the process. Again,
                    // since the process struct does not know about the
                    // grant type we must use a `*mut u8` here.
                    process.set_grant_ptr(grant_num, new_region.as_ptr() as *mut u8);

                    // The allocator returns a `NonNull`, we just want
                    // the raw pointer.
                    new_region.as_ptr()
                } else if untyped_grant_ptr == (!0 as *mut u8) {
                    return Err(Error::AlreadyInUse);
                } else {
                    // Grant region previously allocated, just convert the
                    // pointer.
                    untyped_grant_ptr as *mut T
                };

                Ok(AppliedGrant {
                    process: process,
                    grant_num: grant.grant_num,
                    grant: &mut *(typed_grant_pointer as *mut T),
                    _phantom: PhantomData,
                })
            }
        } else {
            Err(Error::InactiveApp)
        }
    }

    fn get_if_allocated(grant: &Grant<T>, process: &'a dyn ProcessType) -> Option<Self> {
        process.get_grant_ptr(grant.grant_num).and_then(|grant_ptr|
            if grant_ptr.is_null() {
                None
            } else if grant_ptr == (!0 as *mut u8) {
                None
            } else {
                Some(AppliedGrant {
                    process: process,
                    grant_num: grant.grant_num,
                    grant: unsafe { &mut *(grant_ptr as *mut T) },
                    _phantom: PhantomData,
                })
            }
        )
    }

    pub fn enter<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&mut Borrowed<T>, ()) -> R,
        R: Copy,
    {
        let mut root = Borrowed::new(self.grant, self.process.appid());
        unsafe {
            self.process.set_grant_ptr(self.grant_num, !0 as *mut u8);
        }
        let res = fun(&mut root, ());
        unsafe {
            self.process.set_grant_ptr(self.grant_num, root.data as *mut _ as *mut u8);
        }
        res
    }
}

/// Region of process memory reserved for the kernel.
pub struct Grant<T: Default> {
    pub(crate) kernel: &'static Kernel,
    grant_num: usize,
    ptr: PhantomData<T>,
}

impl<T: Default> Grant<T> {
    pub(crate) fn new(kernel: &'static Kernel, grant_index: usize) -> Grant<T> {
        Grant {
            kernel: kernel,
            grant_num: grant_index,
            ptr: PhantomData,
        }
    }

    pub fn enter<F, R>(&self, appid: AppId, fun: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Borrowed<T>, ()) -> R,
        R: Copy,
    {
        appid
            .kernel
            .process_map_or(Err(Error::NoSuchApp), appid, |process| {
                let ag = AppliedGrant::get_or_allocate(self, process)?;
                Ok(ag.enter(fun))
            })
    }

    pub fn each<F>(&self, fun: F)
    where
        F: Fn(&mut Borrowed<T>),
    {
        for ag in self.iter() {
            ag.enter(|borrowed, _| fun(borrowed));
        }
    }

    /// Get an iterator over all processes and their active grant regions for
    /// this particular grant.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            grant: self,
            subiter: self.kernel.get_process_iter(),
        }
    }
}

pub struct Iter<'a, T: 'a + Default> {
    grant: &'a Grant<T>,
    subiter: core::iter::FilterMap<
        core::slice::Iter<'a, Option<&'static dyn ProcessType>>,
        fn(&Option<&'static dyn ProcessType>) -> Option<&'static dyn ProcessType>,
    >,
}

impl<'a, T: Default> Iterator for Iter<'a, T> {
    type Item = AppliedGrant<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let grant = self.grant;
        // Get the next `AppId` from the kernel processes array that is setup to use this grant.
        // Since the iterator itself is saved calling this function
        // again will start where we left off.
        self.subiter.find_map(|process| AppliedGrant::get_if_allocated(grant, process) )
    }
}
