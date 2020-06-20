//! MPU9250 registers and flags

#![allow(dead_code)] // May not use all registers
#![allow(non_camel_case_types)] // Conformance to C, data sheets
#![allow(clippy::identity_op)] // Visually appealing things like 1 << 0

/// The MPU9250's I2C address
pub const I2C_ADDRESS: u8 = 0x68;

/// MPU9250 register addresses
pub mod regs {
    pub const SELF_TEST_X_GYRO: u8 = 0x00;
    pub const SELF_TEST_Y_GYRO: u8 = 0x01;
    pub const SELF_TEST_Z_GYRO: u8 = 0x02;

    pub const SELF_TEST_X_ACCEL: u8 = 0x0D;
    pub const SELF_TEST_Y_ACCEL: u8 = 0x0E;
    pub const SELF_TEST_Z_ACCEL: u8 = 0x0F;

    pub const XG_OFFSET_H: u8 = 0x13;
    pub const XG_OFFSET_L: u8 = 0x14;
    pub const YG_OFFSET_H: u8 = 0x15;
    pub const YG_OFFSET_L: u8 = 0x16;
    pub const ZG_OFFSET_H: u8 = 0x17;
    pub const ZG_OFFSET_L: u8 = 0x18;

    pub const SMPLRT_DIV: u8 = 0x19;
    pub const CONFIG: u8 = 0x1A;
    pub const GYRO_CONFIG: u8 = 0x1B;
    pub const ACCEL_CONFIG: u8 = 0x1C;
    pub const ACCEL_CONFIG_2: u8 = 0x1D;
    pub const LP_ACCEL_ODR: u8 = 0x1F;
    pub const WOM_THR: u8 = 0x1E;
    pub const FIFO_EN: u8 = 0x23;

    pub const I2C_MST_CTRL: u8 = 0x24;

    pub const I2C_SLV0_ADDR: u8 = 0x25;
    pub const I2C_SLV0_REG: u8 = 0x26;
    pub const I2C_SLV0_CTRL: u8 = 0x27;

    pub const I2C_SLV1_ADDR: u8 = 0x28;
    pub const I2C_SLV1_REG: u8 = 0x29;
    pub const I2C_SLV1_CTRL: u8 = 0x2A;

    pub const I2C_SLV2_ADDR: u8 = 0x2B;
    pub const I2C_SLV2_REG: u8 = 0x2C;
    pub const I2C_SLV2_CTRL: u8 = 0x2D;

    pub const I2C_SLV3_ADDR: u8 = 0x2E;
    pub const I2C_SLV3_REG: u8 = 0x2F;
    pub const I2C_SLV3_CTRL: u8 = 0x30;

    pub const I2C_SLV4_ADDR: u8 = 0x31;
    pub const I2C_SLV4_REG: u8 = 0x32;
    pub const I2C_SLV4_DO: u8 = 0x33;
    pub const I2C_SLV4_CTRL: u8 = 0x34;
    pub const I2C_SLV4_DI: u8 = 0x35;

    pub const I2C_MST_STATUS: u8 = 0x36;

    pub const INT_PIN_CFG: u8 = 0x37;
    pub const INT_ENABLE: u8 = 0x38;
    pub const INT_STATUS: u8 = 0x3A;

    pub const ACCEL_XOUT_H: u8 = 0x3B;
    pub const ACCEL_XOUT_L: u8 = 0x3C;
    pub const ACCEL_YOUT_H: u8 = 0x3D;
    pub const ACCEL_YOUT_L: u8 = 0x3E;
    pub const ACCEL_ZOUT_H: u8 = 0x3F;
    pub const ACCEL_ZOUT_L: u8 = 0x40;

    pub const TEMP_OUT_H: u8 = 0x41;
    pub const TEMP_OUT_L: u8 = 0x42;

    pub const GYRO_XOUT_H: u8 = 0x43;
    pub const GYRO_XOUT_L: u8 = 0x44;
    pub const GYRO_YOUT_H: u8 = 0x45;
    pub const GYRO_YOUT_L: u8 = 0x46;
    pub const GYRO_ZOUT_H: u8 = 0x47;
    pub const GYRO_ZOUT_L: u8 = 0x48;

    pub const EXT_SENS_DATA_00: u8 = 0x49;
    pub const EXT_SENS_DATA_01: u8 = 0x4A;
    pub const EXT_SENS_DATA_02: u8 = 0x4B;
    pub const EXT_SENS_DATA_03: u8 = 0x4C;
    pub const EXT_SENS_DATA_04: u8 = 0x4D;
    pub const EXT_SENS_DATA_05: u8 = 0x4E;
    pub const EXT_SENS_DATA_06: u8 = 0x4F;
    pub const EXT_SENS_DATA_07: u8 = 0x50;
    pub const EXT_SENS_DATA_08: u8 = 0x51;
    pub const EXT_SENS_DATA_09: u8 = 0x52;
    pub const EXT_SENS_DATA_10: u8 = 0x53;
    pub const EXT_SENS_DATA_11: u8 = 0x54;
    pub const EXT_SENS_DATA_12: u8 = 0x55;
    pub const EXT_SENS_DATA_13: u8 = 0x56;
    pub const EXT_SENS_DATA_14: u8 = 0x57;
    pub const EXT_SENS_DATA_15: u8 = 0x58;
    pub const EXT_SENS_DATA_16: u8 = 0x59;
    pub const EXT_SENS_DATA_17: u8 = 0x5A;
    pub const EXT_SENS_DATA_18: u8 = 0x5B;
    pub const EXT_SENS_DATA_19: u8 = 0x5C;
    pub const EXT_SENS_DATA_20: u8 = 0x5D;
    pub const EXT_SENS_DATA_21: u8 = 0x5E;
    pub const EXT_SENS_DATA_22: u8 = 0x5F;
    pub const EXT_SENS_DATA_23: u8 = 0x60;

    pub const I2C_SLV0_DO: u8 = 0x63;
    pub const I2C_SLV1_DO: u8 = 0x64;
    pub const I2C_SLV2_DO: u8 = 0x65;
    pub const I2C_SLV3_DO: u8 = 0x66;
    pub const I2C_MST_DELAY_CTRL: u8 = 0x67;
    pub const SIGNAL_PATH_RESET: u8 = 0x68;
    pub const ACCEL_INTEL_CTRL: u8 = 0x69;

    pub const USER_CTRL: u8 = 0x6A;
    pub const PWR_MGMT_1: u8 = 0x6B;
    pub const PWR_MGMT_2: u8 = 0x6C;

    pub const FIFO_COUNTL: u8 = 0x73;
    pub const FIFO_R_W: u8 = 0x74;

    pub const WHO_AM_I: u8 = 0x75;

    pub const XA_OFFSET_H: u8 = 0x77;
    pub const XA_OFFSET_L: u8 = 0x78;
    pub const YA_OFFSET_H: u8 = 0x7A;
    pub const YA_OFFSET_L: u8 = 0x7B;
    pub const ZA_OFFSET_H: u8 = 0x7D;
    pub const ZA_OFFSET_L: u8 = 0x7E;
}

/// MPU9250 flags and register values
pub mod flags {
    use bitflags::bitflags;
    use core::hint;

    /// The flag is used for I2C slave communication. Setting
    /// the 7th bit high indicates a read; setting to 0 indicates
    /// a write.
    pub const I2C_SLV_RNW: u8 = 1 << 7;

    bitflags! {
        #[derive(Default)]
        pub struct FIFO_MODE_FLAG: u8 {
            /// Set FIFO_MODE high to block overwrites to the FIFO
            /// buffer when the FIFO is full. Set low to permit
            /// overwrites of the FIFO, replaing the oldest data
            const FIFO_MODE = 1 << 6;
        }
    }

    /// Enables the FSYNC pin data to be sampled
    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum EXT_SYNC_SET {
        DISABLED = 0,
        TEMP_OUT_L,
        GYRO_XOUT_L,
        GYRO_YOUT_L,
        GYRO_ZOUT_L,
        ACCEL_XOUT_L,
        ACCEL_YOUT_L,
        ACCEL_ZOUT_L,
    }

    impl Default for EXT_SYNC_SET {
        fn default() -> Self {
            EXT_SYNC_SET::DISABLED
        }
    }

    impl From<u8> for EXT_SYNC_SET {
        fn from(byte: u8) -> EXT_SYNC_SET {
            use EXT_SYNC_SET::*;
            match 0b111 & byte {
                0 => DISABLED,
                1 => TEMP_OUT_L,
                2 => GYRO_XOUT_L,
                3 => GYRO_YOUT_L,
                4 => GYRO_ZOUT_L,
                5 => ACCEL_XOUT_L,
                6 => ACCEL_YOUT_L,
                7 => ACCEL_ZOUT_L,
                // three bits may never exceed the range of 0 to 7
                _ => unsafe { hint::unreachable_unchecked() },
            }
        }
    }

    /// Possible selections for digital low pass filters.
    /// The variants are used for gyroscope and accelerometer.
    /// The properties of each selection are qualified in the
    /// docs.
    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum DLPF {
        /// Accelerometer: bandwitdh=218.Hz, delay=1.88ms;
        /// Gyroscope: bandwidth=250Hz, delay=0.97ms, Fs=8kHz;
        /// Temperature sensor: bandwidth=4000 Hz, delay=0.04ms.
        _0 = 0,
        /// Accelerometer: bandwidth=218.1Hz, delay=1.88ms;
        /// Gyroscope: bandwidth=184Hz, delay=2.9ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=188Hz delay=1.9ms.
        _1,
        /// Accelerometer: bandwidth=99Hz, delay=2.88ms;
        /// Gyroscope: bandwidth=92Hz, delay=3.9ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=92Hz, delay=2.8ms.
        _2,
        /// Accelerometer bandwidth=44.8Hz, delay=4.88ms;
        /// Gyroscope: bandwidth=41Hz, delay=5.9ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=42Hz, delay=4.8ms.
        _3,
        /// Accelerometer: bandwidth=21.2Hz, delay=8.87ms;
        /// Gyroscope: bandwidth=20Hz, delay=9.9ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=20Hz, delay=8.3ms.
        _4,
        /// Accelerometer: bandwidth=10.2Hz, delay=16.83ms;
        /// Gyroscope: bandwidth=10Hz, delay=17.85ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=10Hz, delay=13.4ms.
        _5,
        /// Accelerometer: bandwidth=5.05Hz, delay=32.48ms;
        /// Gyroscope: bandwidth=5Hz, delay=33.48ms, Fs=1kHz;
        /// Temperature sensor: bandwidth=5Hz, delay=18.6ms.
        _6,
        /// Accelerometer: bandwidth=420Hz, delay=1.38ms;
        /// Gyroscope: bandwidth=3600Hz, delay=0.17ms, Fs=8kHz;
        /// Temperature sensor: bandwidth=4000Hz, delay=0.04ms.
        _7,
    }

    impl Default for DLPF {
        fn default() -> Self {
            DLPF::_0
        }
    }

    impl From<u8> for DLPF {
        fn from(byte: u8) -> DLPF {
            use DLPF::*;
            match 0b111 & byte {
                0 => _0,
                1 => _1,
                2 => _2,
                3 => _3,
                4 => _4,
                5 => _5,
                6 => _6,
                7 => _7,
                // three bits may never exceed the range of 0 to 7
                _ => unsafe { hint::unreachable_unchecked() },
            }
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct CONFIG {
        fifo_mode: FIFO_MODE_FLAG,
        ext_sync: EXT_SYNC_SET,
        dlpf: DLPF,
    }

    impl From<CONFIG> for u8 {
        fn from(config: CONFIG) -> u8 {
            config.fifo_mode.bits() | ((config.ext_sync as u8) << 3) | (config.dlpf as u8)
        }
    }

    impl From<u8> for CONFIG {
        fn from(byte: u8) -> CONFIG {
            CONFIG {
                fifo_mode: FIFO_MODE_FLAG::from_bits_truncate(byte),
                ext_sync: EXT_SYNC_SET::from(byte),
                dlpf: DLPF::from(byte),
            }
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct GYRO_SELF_TEST: u8 {
            const XGYRO_CTEN = 1 << 7;
            const YGYRO_CTEN = 1 << 6;
            const ZGYRO_CTEN = 1 << 5;
        }
    }

    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum GYRO_FS_SEL {
        DPS250 = 0,
        DPS500,
        DPS1000,
        DPS2000,
    }

    impl Default for GYRO_FS_SEL {
        fn default() -> Self {
            GYRO_FS_SEL::DPS250
        }
    }

    impl From<u8> for GYRO_FS_SEL {
        fn from(byte: u8) -> GYRO_FS_SEL {
            use GYRO_FS_SEL::*;
            match 0b11 & byte {
                0 => DPS250,
                1 => DPS500,
                2 => DPS1000,
                3 => DPS2000,
                // two bits may never exceed the range of 0 to 3
                _ => unsafe { hint::unreachable_unchecked() },
            }
        }
    }

    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum FCHOICE {
        /// Gyro bandwidth: 8800 Hz, delay: 0.064 ms, Fs: 32 kHz
        /// Temperature bandwidth: 4000 Hz, delay: 0.04 ms
        _X0 = 0b01,
        /// Gyro bandwidth: 3600 Hz, delay: 0.11 ms, Fs: 32 kHz
        /// Temperature bandwidth: 4000 Hz, delay: 0.04 ms
        _01 = 0b10,
        /// Enables the DLPF selection
        DLPF = 0b00,
    }

    impl Default for FCHOICE {
        fn default() -> Self {
            FCHOICE::DLPF
        }
    }

    impl From<u8> for FCHOICE {
        fn from(byte: u8) -> FCHOICE {
            match 0b11 & byte {
                0b00 => FCHOICE::DLPF,
                0b01 | 0b11 => FCHOICE::_X0,
                0b10 => FCHOICE::_01,
                _ => unreachable!("two bits high is unrepresentable for FCHOICE"),
            }
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct GYRO_CONFIG {
        self_test: GYRO_SELF_TEST,
        full_scale: GYRO_FS_SEL,
        fchoice: FCHOICE,
    }

    impl From<GYRO_CONFIG> for u8 {
        fn from(config: GYRO_CONFIG) -> u8 {
            config.self_test.bits() | ((config.full_scale as u8) << 3) | (config.fchoice as u8)
        }
    }

    impl From<u8> for GYRO_CONFIG {
        fn from(byte: u8) -> GYRO_CONFIG {
            GYRO_CONFIG {
                self_test: GYRO_SELF_TEST::from_bits_truncate(byte),
                full_scale: GYRO_FS_SEL::from(byte),
                fchoice: FCHOICE::from(byte),
            }
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct ACCEL_SELF_TEST: u8 {
            const AX_ST_EN = 1 << 7;
            const AY_ST_EN = 1 << 6;
            const AZ_ST_EN = 1 << 5;
        }
    }

    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum ACCEL_FS_SEL {
        G2 = 0,
        G4 = 1,
        G8 = 2,
        G16 = 3,
    }

    impl Default for ACCEL_FS_SEL {
        fn default() -> Self {
            ACCEL_FS_SEL::G2
        }
    }

    impl From<u8> for ACCEL_FS_SEL {
        fn from(byte: u8) -> ACCEL_FS_SEL {
            use ACCEL_FS_SEL::*;
            match 0b11 & byte {
                0 => G2,
                1 => G4,
                2 => G8,
                3 => G16,
                // two bits may never exceed the range of 0 to 3
                _ => unsafe { hint::unreachable_unchecked() },
            }
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct ACCEL_CONFIG {
        self_test: ACCEL_SELF_TEST,
        full_scale: ACCEL_FS_SEL,
    }

    impl From<ACCEL_CONFIG> for u8 {
        fn from(config: ACCEL_CONFIG) -> u8 {
            config.self_test.bits() | ((config.full_scale as u8) << 3)
        }
    }

    impl From<u8> for ACCEL_CONFIG {
        fn from(byte: u8) -> ACCEL_CONFIG {
            ACCEL_CONFIG {
                self_test: ACCEL_SELF_TEST::from_bits_truncate(byte),
                full_scale: ACCEL_FS_SEL::from(byte),
            }
        }
    }

    bitflags! {
        /// Write these out to the FIFO at the configured sample rate
        #[derive(Default)]
        pub struct FIFO_EN: u8 {
            const TEMP_OUT  = 1 << 7;
            const GYRO_XOUT = 1 << 6;
            const GYRO_YOUT = 1 << 5;
            const GYRO_ZOUT = 1 << 4;
            const ACCEL     = 1 << 3;
            const SLV2      = 1 << 2;
            const SLV1      = 1 << 1;
            const SLV0      = 1 << 0;
        }
    }

    bitflags! {
        pub struct I2C_MST_CTRL: u8 {
            /// Enables multi-master capability. When disabled, clocking to the I2C_MST_IF
            /// can be disabled when not in use and the logic to detect lost arbitration is
            /// disabled.
            const MULT_MST_EN   = 1 << 7;
            /// Delays the data ready interrupt until external sensor data is loaded. If
            /// I2C_MST_IF is disabled, the interrupt will still occur.
            const WAIT_FOR_ES   = 1 << 6;
            /// 1 – write EXT_SENS_DATA registers associated to SLV_3 (as determined by
            /// I2C_SLV0_CTRL and I2C_SLV1_CTRL and I2C_SLV2_CTRL) to the FIFO at
            /// the sample rate
            ///
            /// 0 – function is disabled
            const SLV_3_FIF_EN  = 1 << 5;
            /// This bit controls the I2C Master’s transition from one slave read to the next
            /// slave read. If 0, there is a restart between reads. If 1, there is a stop between
            /// reads.
            const I2C_MST_P_NSR = 1 << 4;
        }
    }

    bitflags! {
        /// I2C master status
        #[derive(Default)]
        pub struct I2C_MST_STATUS: u8 {
            /// Status of FSYNC interrupt – used as a way to pass an external interrupt
            /// through this chip to the host. If enabled in the INT_PIN_CFG register by
            /// asserting bit FSYNC_INT_EN and if the FSYNC signal transitions from low to
            /// high, this will cause an interrupt. A read of this register clears all status bits
            /// in this register.
            const PASS_THROUGH  = 1 << 7;
            /// Asserted when I2C slave 4’s transfer is complete, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted, and if the
            /// SLV4_DONE_INT_EN bit is asserted in the I2C_SLV4_CTRL register.
            const I2C_SLV4_DONE = 1 << 6;
            /// Asserted when I2C slave looses arbitration of the I2C bus, will cause an
            /// interrupt if bit I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_LOST_ARB  = 1 << 5;
            /// Asserted when slave 4 receives a nack, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_SLV4_NACK = 1 << 4;
            /// Asserted when slave 3 receives a nack, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_SLV3_NACK = 1 << 3;
            /// Asserted when slave 2 receives a nack, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_SLV2_NACK = 1 << 2;
            /// Asserted when slave 1 receives a nack, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_SLV1_NACK = 1 << 1;
            /// Asserted when slave 0 receives a nack, will cause an interrupt if bit
            /// I2C_MST_INT_EN in the INT_ENABLE register is asserted.
            const I2C_SLV0_NACK = 1 << 0;
        }
    }

    bitflags! {
        /// I2C slave 0 through 3 control flags
        ///
        /// Note that there is a separate control flag group for I2C4 slave.
        pub struct I2C_SLVX_CTRL: u8 {
            /// Enable reading data from this slave at the sample rate
            /// and storing data at the first available EXT_SENS_DATA
            /// register
            const I2C_SLVX_EN       = 1 << 7;
            /// 1 – Swap bytes when reading both the low and high byte of
            /// a word. Note there is nothing to swap after reading the first
            /// byte if I2C_SLV1_REG[0] = 1, or if the last byte read has a
            /// register address lsb = 0.
            ///
            /// 0 – no swapping occurs, bytes are written in order read.
            const I2C_SLVX_BYTE_SW  = 1 << 6;
            /// When set, the transaction does not write a register value, it
            /// will only read data, or write data.
            const I2C_SLVX_REG_DIS  = 1 << 5;
            /// External sensor data typically comes in as groups of two bytes. This
            /// bit is used to determine if the groups are from the slave’s register
            /// address 0 and 1, 2 and 3, etc.., or if the groups are address 1 and 2, 3
            /// and 4, etc..
            ///
            /// 0 indicates slave register addresses 0 and 1 are grouped together (odd
            /// numbered register ends the group). 1 indicates slave register
            /// addresses 1 and 2 are grouped together (even numbered register ends
            /// the group). This allows byte swapping of registers that are grouped
            /// starting at any address.
            const I2C_SLVX_GRP      = 1 << 4;
        }
    }

    bitflags! {
        /// I2C slave 4 control flags
        ///
        /// Applies ONLY to slave control register `I2C_SLV4_CTRL`,
        #[derive(Default)]
        pub struct I2C_SLV4_CTRL: u8 {
            /// Enable reading data from this slae at the sample rate
            /// and storing the data at the first available EXT_SENS_DATA
            /// register.
            const I2C_SLV4_EN        = 1 << 7;
            /// Swap bytes when reading both the low and high byte of
            /// a word. Note there is nothing to swap after reading the first
            /// byte if I2C_SLV2_REG[0] = 1, or if the last byte read has a
            /// register address lsb = 0.
            const SLV4_DONE_INT_EN   = 1 << 6;
            /// When set, the transaction does not write a register value, it
            /// will only read data, or write data
            const I2C_SLV4_REG_DIS   = 1 << 5;
        }
    }

    bitflags! {
        /// INT Pin / Bypass Enable Configuration
        #[derive(Default)]
        pub struct INT_PIN_CFG: u8 {
            /// Sets logic level for INT pin is active low (high if not set)
            const ACTL              = 1 << 7;
            /// INT pin is configured as open drain (push pull if not set)
            const OPEN              = 1 << 6;
            /// INT pin level held untilinterrupt status is cleared (cleared within 50us if not set)
            const LATCH_INT_EN      = 1 << 5;
            /// Interrupt status is cleared if any read operation is performed (cleared only by reading INT_STATUS if not set)
            const INT_ANYRD_CLEAR   = 1 << 4;
            /// The logic level for the FSYNC pin as an interrupt is active low (active high if not set)
            const ACTL_FSYNC        = 1 << 3;
            /// This enables the FSYNC pin to be used as an interrupt.
            /// A transition to the active level described by the ACTL_FSYNC bit
            /// will cause an interrupt.  The status of the interrupt is read in
            /// the I2C Master Status register PASS_THROUGH bit (disabled if not set)
            const FSYNC_INT_MODE_EN = 1 << 2;
            /// When asserted, the i2c_master interface pins(ES_CL and ES_DA) will
            /// go into ‘bypass mode’ when the i2c master interface is disabled.
            /// The pins will float high due to the internal pull-up if not enabled
            /// and the i2c master interface is disabled
            const BYPASS_EN         = 1 << 1;
        }
    }

    bitflags! {
        /// Enable interrupt for...
        #[derive(Default)]
        pub struct INT_ENABLE: u8 {
            /// 1 – Enable interrupt for wake on motion to propagate to interrupt pin.
            const WOM_EN            = 1 << 6;
            /// 1 – Enable interrupt for fifo overflow to propagate to interrupt pin.
            const FIFO_OVERFLOW_EN  = 1 << 4;
            /// 1 – Enable Fsync interrupt to propagate to interrupt pin.
            const FSYNC_INT_EN      = 1 << 3;
            /// 1 – Enable Raw Sensor Data Ready interrupt to propagate to interrupt pin.
            /// The timing of the interrupt can vary depending on the setting in
            /// register 36 I2C_MST_CTRL, bit [6] WAIT_FOR_ES
            const RAW_RDY_EN        = 1 << 0;
        }
    }

    bitflags! {
        /// Interrupt status
        #[derive(Default)]
        pub struct INT_STATUS: u8 {
            /// Wake on motion interrupt
            const WOM_INT           = 1 << 6;
            /// Fifo Overflow interrupt occurred. Note that the oldest data is has
            /// been dropped from the fifo.
            const FIFO_OVERFLOW_INT = 1 << 4;
            /// FSYNC interrupt occured
            const FSYNC_INT         = 1 << 3;
            /// Sensor Register Raw Data sensors are updated and Ready to be
            /// read. The timing of the interrupt can vary depending on the setting in
            /// register 36 I2C_MST_CTRL, bit [6] WAIT_FOR_ES.
            const RAW_DATA_RDY_INT  = 1 << 0;
        }
    }

    bitflags! {
        /// I2C master delay control
        #[derive(Default)]
        pub struct I2C_MST_DELAY_CTRL: u8 {
            /// Delays shadowing of external sensor data until all data is received
            const DELAY_ES_SHADOW = 1 << 7;
            /// When enabled, slave X will only be accessed (1+I2C_MST_DLY) samples
            /// as determined by SMPLRT_DIV and DLPF_CFG
            const I2C_SLV4_DLY_EN = 1 << 4;
            const I2C_SLV3_DLY_EN = 1 << 3;
            const I2C_SLV2_DLY_EN = 1 << 2;
            const I2C_SLV1_DLY_EN = 1 << 1;
            const I2C_SLV0_DLY_EN = 1 << 0;
        }
    }

    bitflags! {
        /// Signal path reset
        #[derive(Default)]
        pub struct SIGNAL_PATH_RESET: u8 {
            const GYRO_RST  = 0b100;
            const ACCEL_RST = 0b010;
            const TEMP_RST  = 0b001;
        }
    }

    bitflags! {
        /// Accelerometer input control
        #[derive(Default)]
        pub struct ACCEL_INTEL_CTRL: u8 {
            /// Enables the Wake-on-Motion detection logic.
            const ACCEL_INTEL_EN    = 1 << 7;
            /// 1 = Compare the current sample with the previous sample.
            /// 0 = Not used.
            const ACCEL_INTEL_MODE  = 1 << 6;
        }
    }

    bitflags! {
        /// User control
        #[derive(Default)]
        pub struct USER_CTRL: u8 {
            /// 1 - Enable FIFO
            ///
            /// 0 - Disable FIFO access from serial interface. To disable FIFO writes by
            /// dma, use FIFO_EN register. To disable possible FIFO writes from DMP,
            /// disable the DMP.
            const FIFO_EN       = 1 << 6;
            /// 1 – Enable the I2C Master I/F module; pins ES_DA and ES_SCL are isolated
            /// from pins SDA/SDI and SCL/ SCLK.
            ///
            /// 0 – Disable I2C Master I/F module; pins ES_DA and ES_SCL are logically
            /// driven by pins SDA/SDI and SCL/ SCLK.
            ///
            /// NOTE: DMP will run when enabled, even if all internal sensors are disabled,
            /// except when the sample rate is set to 8Khz.
            const I2C_MST_EN    = 1 << 5;
            /// 1 – Disable I2C Slave module and put the serial interface in SPI mode only.
            const I2C_IF_DIS    = 1 << 4;
            /// 1 – Reset FIFO module. Reset is asynchronous. This bit auto clears after
            /// one clock cycle.
            const FIFO_RST      = 1 << 2;
            /// 1 – Reset I2C Master module. Reset is asynchronous. This bit auto clears
            /// after one clock cycle.
            ///
            /// NOTE: This bit should only be set when the I2C master has hung. If this bit
            /// is set during an active I2C master transaction, the I2C slave will hang, which
            /// will require the host to reset the slave.
            const I2C_MST_RST   = 1 << 1;
            /// 1 – Reset all gyro digital signal path, accel digital signal path, and temp
            /// digital signal path. This bit also clears all the sensor registers.
            /// SIG_COND_RST is a pulse of one clk8M wide.
            const SIG_COND_RST  = 1 << 0;
        }
    }

    bitflags! {
        /// Power management
        #[derive(Default)]
        pub struct PWR_MGMT_1: u8 {
            /// 1 – Reset the internal registers and restores the default settings. Write a 1 to
            /// set the reset, the bit will auto clear.
            const H_RESET       = 1 << 7;
            /// When set, the chip is set to sleep mode (After OTP loads, the
            /// PU_SLEEP_MODE bit will be written here)
            const SLEEP         = 1 << 6;
            /// When set, and SLEEP and STANDBY are not set, the chip will cycle
            /// between sleep and taking a single sample at a rate determined by
            /// LP_ACCEL_ODR register
            ///
            /// NOTE: When all accelerometer axis are disabled via PWR_MGMT_2
            /// register bits and cycle is enabled, the chip will wake up at the rate
            /// determined by the respective registers above, but will not take any samples.
            const CYCLE         = 1 << 5;
            /// When set, the gyro drive and pll circuitry are enabled, but the sense paths
            /// are disabled. This is a low power mode that allows quick enabling of the
            /// gyros
            const GYRO_STANDBY  = 1 << 4;
            /// Power down internal PTAT voltage generator and PTAT ADC
            const PD_PTAT       = 1 << 3;
        }
    }

    bitflags! {
        /// Set these flags to disable sensors and axes
        #[derive(Default)]
        pub struct PWR_MGMT_2: u8 {
            const DISABLE_XA    = 1 << 5;
            const DISABLE_YA    = 1 << 4;
            const DISABLE_ZA    = 1 << 3;

            const DISABLE_XG    = 1 << 2;
            const DISABLE_YG    = 1 << 1;
            const DISABLE_ZG    = 1 << 0;
        }
    }
}
