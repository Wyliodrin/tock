use kernel::commom::registers::{register_bitfields, register_structs};

register_structs! {
    CanRegisters {
        
        /// CAN control and status registers
        (0x000 => can_mcr: [ReadWrite<u32, CAN_MCR::Register>; 32]),
        /// CAN master status register
        (0x004 => can_msr: [ReadWrite<u32, CAN_MSR::Register>; 32]),
        /// CAN transmit status register
        (0x008 => can_tsr: [ReadWrite<u32, CAN_TSR::Register>; 32]),
        /// CAN receive FIFO 0 register
        (0x00c => can_rf0r: [ReadWrite<u32, CAN_RF0R::Register>; 32]),
        /// CAN receive FIFO 1 register
        (0x010 => can_rf1r: [ReadWrite<u32, CAN_RF1R::Register>; 32]),
        /// CAN interrupt enable register
        (0x014 => can_ier: [ReadWrite<u32, CAN_IER::Register>; 32]),
        /// CAN error status register
        (0x018 => can_esr: [ReadWrite<u32, CAN_ESR::Register>; 32]),
        /// CAN bit timing register
        (0x01c => can_btr: [ReadWrite<u32, CAN_BTR::Register>; 32]),
        ///
        /// 
        /// CAN MAILBOX REGISTERS
        /// 
        /// 
        /// CAN TX mailbox identifier register
        (0x020 => _reserved0),

        (0x180 => can_ti0r: [ReadWrite<u32, CAN_TIxR::Register>; 32]),
        /// CAN mailbox data length control and time stamp register
        (0x184 => can_tdt0r: [ReadWrite<u32, CAN_TDTxR::Register>; 32]),
        /// CAN mailbox data low register
        (0x188 => can_tdl0r: [ReadWrite<u32, CAN_TDLxR::Register>; 32]),
        /// CAN mailbox data high register
        (0x18c => can_tdh0r: [ReadWrite<u32, CAN_TDHxR::Register>; 32]),
        /// CAN receive FIFO mailbox identifier register
        ( => can_rixr: [ReadWrite<u32, CAN_RIxR::Register>; 32]),
        /// CAN receive FIFO mailbox data length control and timpe stamp register
        ( => can_rdtxr: [ReadWrite<u32, CAN_RDTxR::Register>; 32]),
        /// CAN receive FIFO mailbox data low register
        ( => can_rdlxr: [ReadWrite<u32, CAN_RDLxR::Register>; 32]),
        /// CAN receive FIFO mailbox data high register
        ( => can_rdhxr: [ReadWrite<u32, CAN_RDHxR::Register>; 32]),
        /// 
        /// 
        /// CAN FILTER REGISTERS
        /// 
        /// 
        /// CAN filter master register
        ( => can_fmr: [ReadWrite<u32, CAN_FMR::Register>; 32]),
        /// CAN filter mode register
        ( => can_fm1r: [ReadWrite<u32, CAN_FM1R::Register>; 32]),
        /// CAN filter scale register
        ( => can_fs1r: [ReadWrite<u32, CAN_FS1R::Register>; 32]),
        /// CAN filter FIFO assignment register
        ( => can_ffa1r: [ReadWrite<u32, CAN_FFA1R::Register>; 32]),
        /// CAN filter activation register
        ( => can_fa1r: [ReadWrite<u32, CAN_FA1R::Register>; 32]),
    }
}

register_bitfields![u32,
    CAN_MCR [
        /// Debug freeze
        DBF OFFSET(16) NUMBITS(1) [],
        /// bcXAN software master reset
        RESET OFFSET(15) NUMBITS(1) [],
        /// Time triggered communication mode
        TTCM OFFSET(7) NUMBITS(1) [],
        /// Automatic bus-off management
        ABOM OFFSET(6) NUMBITS(1) [],
        /// Automatic wakeup mode
        AWUM OFFSET(5) NUMBITS(1) [],
        /// No automatic retransmission
        NART OFFSET(4) NUMBITS(1) [],
        /// Receive FIFO locked mode
        RFLM OFFSET(3) NUMBITS(1) [],
        /// Transmit FIFO prioritY
        TXFP OFFSET(2) NUMBITS(1) [],
        /// Sleep mode request
        SLEEP OFFSET(1) NUMBITS(1) [],
        /// Initialization request
        IRQN OFFSET(0) NUMBITS(1) []
    ],
    CAN_MSR [
        /// CAN Rx signal
        RX OFFSET(11) NUMBITS(1) [],
        /// Last sample point
        SAMP OFFSET(10) NUMBITS(1) [],
        /// Receive mode
        RXM OFFSET(9) NUMBITS(1) [],
        /// Transmit mode
        TXM OFFSET(8) NUMBITS(1) [],
        /// Sleep acknowledge interrupt
        SLAKI OFFSET(4) NUMBITS(1) [],
        /// Wakeup interrupt
        WKUI OFFSET(3) NUMBITS(1) [],
        /// Error interrupt
        ERRI OFFSET(2) NUMBITS(1) [],
        /// Sleep acknowledge
        SLAK OFFSET(1) NUMBITS(1) [],
        /// Initialization acknowledge
        INAK OFFSET(0) NUMBITS(1) []
    ],
    CAN_TSR [
        /// Lowest priority flag for mailbox 2
        LOW2 OFFSET(31) NUMBITS(1) [],
        /// Lowest priority flag for mailbox 1
        LOW1 OFFSET(30) NUMBITS(1) [],
        /// Lowest priority flag for mailbox 0
        LOW0 OFFSET(29) NUMBITS(1) [],
        /// Transmit mailbox 2 empty
        TME2 OFFSET(28) NUMBITS(1) [],
        /// Transmit mailbox 1 empty
        TME1 OFFSET(27) NUMBITS(1) [], 
        /// Transmit mailbox 0 empty
        TME0 OFFSET(26) NUMBITS(1) [],
        /// Mailbox code
        CODE OFFSET(24) NUMBITS(2) [],
        /// Abort request for mailbox 2
        ABRQ2 OFFSET(23) NUMBITS(1) [],
        /// Transmission error of mailbox 2
        TERR2 OFFSET(19) NUMBITS(1) [],
        /// Arbitration lost for mailbox 2
        ALST2 OFFSET(18) NUMBITS(1) [],
        /// Transmission OK of mailbox 2
        TXOK2 OFFSET(17) NUMBITS(1) [],
        /// Request completed mailbox 2
        RQCP2 OFFSET(16) NUMBITS(1) [],
        /// Abort request for mailbox 1
        ABRQ1 OFFSET(15) NUMBITS(1) [],
        /// Transmission error of mailbox 1
        TERR1 OFFSET(11) NUMBITS(1) [],
        /// Arbitration lost for mailbox 1
        ALST1 OFFSET(10) NUMBITS(1) [],
        /// Transmission OK of mailbox 1
        TXOK1 OFFSET(9) NUMBITS(1) [],
        /// Request completed mailbox 1
        RQCP1 OFFSET(8) NUMBITS(1) [],
        /// Abort request for mailbox 0
        ABRQ0 OFFSET(7) NUMBITS(1) [],
        /// Transmission error of mailbox 0
        TERR0 OFFSET(3) NUMBITS(1) [],
        /// Arbitration lost for mailbox 0
        ALST0 OFFSET(2) NUMBITS(1) [],
        /// Transmission OK of mailbox 0
        TXOK0 OFFSET(1) NUMBITS(1) [],
        /// Request completed mailbox 0
        RQCP0 OFFSET(0) NUMBITS(1) []
    ],
    CAN_RF0R [
        /// Release FIFO 0 output mailbox
        RFOM0 OFFSET(5) NUMBITS(1) [],
        /// FIFO 0 overrun
        FOVR0 OFFSET(4) NUMBITS(1) [],
        /// FIFO 0 full
        FULL0 OFFSET(3) NUMBITS(1) [],
        /// FIFO 0 message pending
        FMP0 OFFSET(0) NUMBITS(2) []
    ],
    CAN_RF1R [
        /// Release FIFO 1 output mailbox
        RFOM1 OFFSET(5) NUMBITS(1) [],
        /// FIFO 1 overrun
        FOVR1 OFFSET(4) NUMBITS(1) [],
        /// FIFO 1 full
        FULL1 OFFSET(3) NUMBITS(1) [],
        /// FIFO 1 message pending
        FMP1 OFFSET(0) NUMBITS(2) []
    ],
    CAN_IER [
        /// Sleep interrupt enable
        SLKIE OFFSET(17) NUMBITS(1) [],
        /// Wakeup interrupt enable
        WKUIE OFFSET(16) NUMBITS(1) [],
        /// Error interrupt enable
        ERRIE OFFSET(15) NUMBITS(1) [],
        /// Last error code interrupt enable
        LECIE OFFSET(11) NUMBITS(1) [],
        /// Bus-off interrupt enable
        BOFIE OFFSET(10) NUMBITS(1) [],
        /// Error passive interrupt enable
        EPVIE OFFSET(9) NUMBITS(1) [],
        /// Error warning interrupt enable
        EWGIE OFFSET(8) NUMBITS(1) [],
        /// FIFO 1 overrun interrupt enable
        FOVIE1 OFFSET(6) NUMBITS(1) [],
        /// FIFO 1 full interrupt enable
        FFIE1 OFFSET(5) NUMBITS(1) [],
        /// FIFO 1 message pending interrupt enable
        FMPIE1 OFFSET(4) NUMBITS(1) [],
        /// FIFO 0 overrun interrupt enable
        FOVIE0 OFFSET(3) NUMBITS(1) [],
        /// FIFO 0 full interrupt enable
        FFIE0 OFFSET(2) NUMBITS(1) [],
        /// FIFO 0 message pending interrupt enable
        FMPIE0 OFFSET(1) NUMBITS(1) [],
        /// Transmit mailbox empty interrupt enable
        TMEIE OFFSET(0) NUMBITS(1) [],
    ],
    CAN_ESR [
        /// Receive error counter
        REC OFFSET(24) NUMBITS(8) [],
        /// Least significant byte of the 9-bit transmit error counter
        TEC OFFSET(16) NUMBITS(8) [],
        /// Last error code
        LEC OFFSET(4) NUMBITS(3) [],
        /// Bus-off flag
        BOFF OFFSET(2) NUMBITS(1) [],
        /// Error passive flag
        EPVF OFFSET(1) NUMBITS(1) [],
        /// Error warning flag
        EWGF OFFSET(0) NUMBITS(1) []
    ],
    CAN_BTR [
        /// Silent mode (debug)
        SILM OFFSET(31) NUMBITS(1) [],
        /// Loop back mode (debug)
        LBKM OFFSET(30) NUMBITS(1) [],
        /// Resynchronization jump width
        SJW OFFSET(24) NUMBITS(2) [],
        /// Time segment 2
        TS2 OFFSET(20) NUMBITS(3) [],
        /// Time segment 1
        TS1 OFFSET(16) NUMBITS(4) [],
        /// Baud rate prescaler
        BRP OFFSET(0) NUMBITS(10) []
    ],
    ///
    /// 
    /// CAN mailbox registers
    /// 
    /// 
    CAN_
    /// 
    /// 
    /// CAN filter registers
    /// 
    /// 
    CAN_FMR [
        /// CAN start bank
        CANSB OFFSET(8) NUMBITS(6) [],
        /// Filter initialization mode
        FINIT OFFSET(0) NUMBITS(1), []
    ],
    CAN_FM1R [
        
    ],
    CAN_FS1R [

    ],
    CAN_FFA1R [

    ],
    CAN_FA1R [

    ],
];