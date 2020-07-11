//! Registers for the on-board AK8963

#![allow(dead_code)] // May not use all registers
#![allow(non_camel_case_types)] // Conformance to C, data sheets
#![allow(clippy::identity_op)] // Visually appealing things like 1 << 0

/// The AK8963's I2C address
///
/// This may only be addressable when the MPU9250 is in
/// I2C pass-through mode.
pub const I2C_ADDRESS: u8 = 0x0C;

/// Possible responses for `WHO_AM_I`
pub static VALID_WHO_AM_I: &[u8] = &[0x48];

/// AK8963 register addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Regs {
    WIA = 0x00,
    INFO = 0x01,
    ST1 = 0x02,

    HXL = 0x03,
    HXH = 0x04,
    HYL = 0x05,
    HYH = 0x06,
    HZL = 0x07,
    HZH = 0x08,
    ST2 = 0x09,

    CNTL1 = 0x0A,
    CNTL2 = 0x0B,
    ASTC = 0x0C,

    I2CDIS = 0x0F,
    ASAX = 0x10,
    ASAY = 0x11,
    ASAZ = 0x12,
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
        /// Take one measurement, then power down
        SINGLE_MEASUREMENT = 0b0001,
        /// Sample at 8Hz
        CONTINUOUS_1 = 0b0010,
        /// Sample at 100Hz
        CONTINUOUS_2 = 0b0110,
        /// Sample on rising-edge on TRG pin
        EXTERNAL_TRG = 0b0100,
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
        pub mode: CNTL1_MODE,
        pub output: CNTL1_OUTPUT,
    }

    impl CNTL1 {
        pub fn power_down() -> Self {
            CNTL1::default()
        }
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
                0b0100 => CNTL1_MODE::EXTERNAL_TRG,
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
