//! Registers for the on-board AK8963

#![allow(dead_code)] // May not use all registers
#![allow(non_camel_case_types)] // Conformance to C, data sheets
#![allow(clippy::identity_op)] // Visually appealing things like 1 << 0

/// The AK8963's I2C address
///
/// This may only be addressable when the MPU9250 is in
/// I2C pass-through mode.
pub const I2C_ADDRESS: u8 = 0x0C;

/// AK8963 register addresses
pub mod regs {
    pub const WIA: u8 = 0x00;
    pub const INFO: u8 = 0x01;
    pub const ST1: u8 = 0x02;

    pub const HXL: u8 = 0x03;
    pub const HXH: u8 = 0x04;
    pub const HYL: u8 = 0x05;
    pub const HYH: u8 = 0x06;
    pub const HZL: u8 = 0x07;
    pub const HZH: u8 = 0x08;
    pub const ST2: u8 = 0x09;

    pub const CNTL1: u8 = 0x0A;
    pub const CNTL2: u8 = 0x0B;
    pub const ASTC: u8 = 0x0C;

    pub const I2CDIS: u8 = 0x0F;
    pub const ASAX: u8 = 0x10;
    pub const ASAY: u8 = 0x11;
    pub const ASAZ: u8 = 0x12;
}

/// AK8963 flags and register values
pub mod flags {
    use bitflags::bitflags;

    bitflags! {
        /// Status 1 flags
        #[derive(Default)]
        pub struct ST1 : u8 {
            /// Data read when high
            const DRDY  = 0b0000_0001;
            /// Data overrun when high
            const DOR   = 0b0000_0010;
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct ST2: u8 {
            /// Magnetic sensor overflow
            const HOFL  = 0b0000_0100;
            /// Mirror of BIT in CNTL1
            const BITM  = 0b0001_0000;
        }
    }

    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum CNTL1_MODE {
        POWER_DOWN = 0b0000,
        SINGLE_MEASUREMENT = 0b0001,
        CONTINUOUS_1 = 0b0010,
        CONTINUOUS_2 = 0b0110,
        SELF_TEST = 0b1000,
        FUSE_ROM_ACCESS = 0b1111,
    }

    impl Default for CNTL1_MODE {
        fn default() -> Self {
            CNTL1_MODE::POWER_DOWN
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct CNTL1_OUTPUT: u8 {
            /// Set for 16-bit output, else 14 bit
            const SIXTEEN_BIT_EN = 1 << 6;
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct CNTL1 {
        mode: CNTL1_MODE,
        output: CNTL1_OUTPUT,
    }

    impl From<CNTL1> for u8 {
        fn from(cntl: CNTL1) -> u8 {
            cntl.output.bits() | cntl.mode as u8
        }
    }

    impl From<u8> for CNTL1 {
        fn from(byte: u8) -> CNTL1 {
            let mode = match 0b1111 & byte {
                0b0000 => CNTL1_MODE::POWER_DOWN,
                0b0001 => CNTL1_MODE::SINGLE_MEASUREMENT,
                0b0010 => CNTL1_MODE::CONTINUOUS_1,
                0b0110 => CNTL1_MODE::CONTINUOUS_2,
                0b1000 => CNTL1_MODE::SELF_TEST,
                0b1111 => CNTL1_MODE::FUSE_ROM_ACCESS,
                _ => unreachable!("found unexepected value when handling CNTL1 byte"),
            };
            CNTL1 {
                output: CNTL1_OUTPUT::from_bits_truncate(byte),
                mode,
            }
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct CNTL2: u8 {
            const SRST = 1 << 0;
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct ASTC: u8 {
            const SELF = 1 << 6;
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct I2CDIS;

    impl I2CDIS {
        /// Magic number to disable I2C
        const MAGIC: u8 = 0b00011011;
    }

    impl From<I2CDIS> for u8 {
        fn from(_: I2CDIS) -> u8 {
            I2CDIS::MAGIC
        }
    }

    impl From<u8> for I2CDIS {
        fn from(byte: u8) -> I2CDIS {
            debug_assert!(byte == I2CDIS::MAGIC);
            I2CDIS
        }
    }
}
