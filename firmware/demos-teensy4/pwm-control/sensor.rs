//! Motion sensor serialization

use core::time::Duration;
use embedded_hal::{
    blocking::i2c::{Write, WriteRead},
    timer::{CountDown, Periodic},
};
use invensense_mpu::Mpu;
use motion_sensor::*;

const POLLING_INTERVAL: Duration = Duration::from_micros(10_000);

pub struct Sensor<P, I> {
    timer: P,
    write: crate::Datapath,
    mpu: Option<Mpu<invensense_mpu::i2c::Bypass<I>>>,
}

impl<P, I, E> Sensor<P, I>
where
    P: CountDown<Time = Duration> + Periodic,
    I: WriteRead<Error = E> + Write<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(
        mut timer: P,
        i2c: I,
        write: crate::Datapath,
        blocking: &mut dyn embedded_hal::blocking::delay::DelayMs<u8>,
    ) -> Self {
        let mut config = invensense_mpu::Config::default();
        config.accel_scale = invensense_mpu::regs::ACCEL_FS_SEL::G8;
        config.mag_control = invensense_mpu::regs::CNTL1 {
            mode: invensense_mpu::regs::CNTL1_MODE::CONTINUOUS_2,
            ..Default::default()
        };

        let mpu = match invensense_mpu::i2c::new(i2c, blocking, &config) {
            Ok(mpu) => Some(mpu),
            Err(err) => {
                log::warn!("Could not find MPU9250: {:?}", err);
                None
            }
        };
        timer.start(POLLING_INTERVAL);
        Sensor { timer, write, mpu }
    }

    pub fn is_active(&self) -> bool {
        self.mpu.is_some()
    }

    pub fn poll(&mut self) {
        if let Some(mpu) = &mut self.mpu {
            macro_rules! _try {
                ($e:expr) => {
                    match $e {
                        Err(err) => {
                            log::warn!("{:?}", err);
                            return;
                        }
                        Ok(ok) => ok,
                    }
                };
            }

            _try!(self.write.poll());
            if let Ok(()) = self.timer.wait() {
                let (acc, gyro, mag) = _try!(mpu.marg());
                let readings = [
                    Reading::Accelerometer(acc),
                    Reading::Gyroscope(gyro),
                    Reading::Magnetometer(mag),
                ];
                let mut buffer = [0; 128];
                let offset = _try!(postcard::to_slice_cobs(&readings[..], &mut buffer)).len();
                _try!(self.write.write(&buffer[..offset]));
            }
        }
    }
}
