//! PWM control example
//!
//! # Motor control
//!
//! The example lets us change PWM outputs from user input. There are four
//! PWM outputs, identified by the letters A through D:
//!
//! | PWM Output | Teensy 4 Pin | PWM instance |
//! | ---------- | ------------ | ------------ |
//! |     C      |      6       |  `PWM2_2_A`  |
//! |     B      |      7       |  `PWM1_3_B`  |
//! |     A      |      8       |  `PWM1_3_A`  |
//! |     D      |      9       |  `PWM2_2_B`  |
//!
//! To set the throttle commanded by a PWM output, use the addressing schema
//!
//! ```text
//! O.ppp\r
//! ````
//!
//! where
//!
//! - `O` is one of the four output letters
//! - `ppp` is a throttle percentage from 0 to 100. The software will require inputs within this range.
//! - `\r` is a carriage return character
//!
//! Example: to set the throttle for output `C` to 37%, type `C.37`, then press `ENTER` on your
//! keyboard.
//!
//! If you start to enter an invalid number, just press ENTER to submit it, and let the parser fail.
//!
//! To read back the throttle for all outputs, send 'r' (lower case 'R').
//!
//! Press the SPACE bar to reset all throttles to 0%. Use this in case of emergency...
//!
//! The demo implements a basic electronic speed control (ESC) PWM driver. The PWM duty cycle varies between
//! 1000us for 0% throttle to 2000us for 100% throttle speed.
//!
//! ```text
//!      1000us              2000us      1000us          1000us          1000us      
//!     +-------+       +--------------++-------+       +-------+       +-------+    
//!     |       |       |              ||       |       |       |       |       |    
//!     |       |       |              ||       |       |       |       |       |    
//!     |       |       |              ||       |       |       |       |       |    
//! ----+       +-------+              ++       +-------+       +-------+       +---...
//! ```
//!
//! _(Above) a quick pulse to 100% throttle in between commands to hold 0% throttle_
//!
//! Given that we need to hold a 2000us pulse to represent 100% throttle, the fastest switching frequency
//! is 1 / 2000us == 500Hz. At 500Hz switching frequency, 100% throttle maps to 100% duty cycle. Therefore,
//! 50% duty cycle represents 0% throttle. We're assuming that the ESC
//! can perfectly detect a 2000us pulse followed immediately by another 2000us pulse, where there may be
//! no delay between the two pulses.
//!
//! # Motion Sensor
//!
//! You can optionally connect an MPU9250 motion sensor to a Teensy 4's I2C peripheral. If connected,
//! the example will poll the sensor and write the data over a UART peripheral. The table below describes
//! the I2C sensor and UART pinouts.
//!
//! | Teensy 4 Pin | Teensy 4 Function |  Connection  |
//! | ------------ | ----------------- | ------------ |
//! |      14      |     UART2 TX      | Host UART RX |
//! |      15      |     UART2 RX      | Host UART TX |
//! |      16      |     I2C3 SCL      |   MPU SCL    |
//! |      17      |     I2C3 SDA      |   MPU SDA    |
//!
//! Note that the motion sensor is optional. If the sensor is not connected, you may still control motors.
//!
//! IMU readings represent a COBS-encoded slice of one ore more `motion_sensor::Reading` measurements. You
//! may deserialize them using `postcard`.

#![no_std]
#![no_main]

mod datapath;
mod parser;
mod sensor;

extern crate panic_halt;

use bsp::hal::i2c::ClockSpeed;
use core::time::Duration;
use cortex_m_rt::entry;
use embedded_hal::{digital::v2::OutputPin, timer::CountDown};
use parser::{Command, Parser};
use teensy4_bsp as bsp;

use esc::{QuadMotor, ESC};
use esc_imxrt1062::{Protocol, ESC as imxrtESC};

/// CHANGE ME to vary the ESC protocol
const ESC_PROTOCOL: Protocol = Protocol::OneShot125;
const I2C_CLOCK_SPEED: ClockSpeed = ClockSpeed::KHz400;
const UART_BAUD: u32 = 115_200;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(core_peripherals.SYST);
    let pins = bsp::t40::into_pins(peripherals.iomuxc);

    let mut led = bsp::configure_led(pins.p13);

    // Initialize the ARM and IPG clocks. The PWM module runs on the IPG clock.
    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    let mut pit_cfg = peripherals.ccm.perclk.configure(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );
    let (mut led_timer, _, _, sensor_timer) = peripherals.pit.clock(&mut pit_cfg);

    // Enable clocks to the PWM modules
    let mut pwm1 = peripherals.pwm1.clock(&mut peripherals.ccm.handle);
    let mut pwm2 = peripherals.pwm2.clock(&mut peripherals.ccm.handle);

    // Dummy value; will be reconfigured by the ESC implementation
    let switching_period = Duration::from_micros(5_000);

    // Configure pins 6 and 9 as PWM outputs
    let sm2 = pwm2
        .sm2
        .outputs(
            &mut pwm2.handle,
            pins.p6,
            pins.p9,
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap();

    // Configure pins 7 and 8 as PWM outputs
    let sm3 = pwm1
        .sm3
        .outputs(
            &mut pwm1.handle,
            pins.p8,
            pins.p7,
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap();

    let mut esc = imxrtESC::new(ESC_PROTOCOL, pwm1.handle, sm3, pwm2.handle, sm2);

    // Set up the USB stack, and use the USB reader for parsing commands
    let usb_reader = bsp::usb::init(&systick, Default::default()).unwrap();
    let mut parser = Parser::new(usb_reader);

    let blink_period = pwm_to_blink_period(&esc);
    led_timer.start(blink_period);

    // ----------
    // UART setup
    // ----------
    let uarts = peripherals.uart.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::uart::ClockSelect::OSC,
        bsp::hal::ccm::uart::PrescalarSelect::DIVIDE_1,
    );
    let uart = uarts.uart2.init(pins.p14, pins.p15, UART_BAUD).unwrap();
    let (tx, _) = uart.split();

    // ---------
    // DMA setup
    // ---------
    let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
    let channel_7 = dma_channels[7].take().unwrap();

    // --------------
    // Datapath setup
    // --------------
    let datapath = match datapath::Datapath::new(tx, channel_7) {
        Ok(datapath) => datapath,
        Err(err) => {
            log::error!("Unable to establish datapath: {:?}", err);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
    };

    // ---------
    // I2C setup
    // ---------
    let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // Use the 24MHz oscillator as the peripheral clock source
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide that 24MHz clock by 3
    );
    let mut i2c3 = i2c3_builder.build(pins.p16, pins.p17);
    // Set the I2C clock speed. If this returns an error, log it and stop.
    match i2c3.set_clock_speed(I2C_CLOCK_SPEED) {
        Ok(_) => (),
        Err(err) => {
            log::error!(
                "Unable to set I2C clock speed to {:?}: {:?}",
                I2C_CLOCK_SPEED,
                err
            );
            loop {
                // Nothing more to do
                core::sync::atomic::spin_loop_hint();
            }
        }
    }

    // ------------
    // Sensor setup
    // ------------
    let mut sensor = sensor::Sensor::new(sensor_timer, i2c3, datapath, &mut systick);

    log::info!("=============READY=============");
    loop {
        if let Ok(()) = led_timer.wait() {
            led.toggle();
        }

        match parser.parse() {
            // Parser has not found any command; it needs more inputs
            Ok(None) => sensor.poll(),
            // User wants to reset all duty cycles
            Ok(Some(Command::ResetThrottle)) => {
                esc.set_throttle_group(&[
                    (QuadMotor::A, 0.0),
                    (QuadMotor::B, 0.0),
                    (QuadMotor::C, 0.0),
                    (QuadMotor::D, 0.0),
                ]);
                log::info!("Reset all outputs to 0% throttle");
                let blink_period = pwm_to_blink_period(&esc);
                led_timer.start(blink_period);
            }
            // User wants to read all the throttle settings
            Ok(Some(Command::ReadSettings)) => {
                log::info!("ESC_PROTOCOL = {:?}", ESC_PROTOCOL);
                log::info!(
                    "SENSOR = {}",
                    if sensor.is_active() {
                        "CONNECTED"
                    } else {
                        "DISCONNECTED"
                    }
                );
                log::info!("A = {}", esc.throttle(QuadMotor::A) * 100.0);
                log::info!("B = {}", esc.throttle(QuadMotor::B) * 100.0);
                log::info!("C = {}", esc.throttle(QuadMotor::C) * 100.0);
                log::info!("D = {}", esc.throttle(QuadMotor::D) * 100.0);
            }
            // User has set a throttle for an output
            Ok(Some(Command::SetThrottle { output, percent })) => {
                log::info!("SETTING '{:?}' = {}% throttle", output, percent);
                esc.set_throttle(output, percent / 100.0);

                let blink_period = pwm_to_blink_period(&esc);
                led_timer.start(blink_period);
            }
            Ok(Some(Command::KillSwitch)) => {
                esc.kill();

                log::warn!("------------------------------------");
                log::warn!("USER PRESSED THE KILL SWITCH");
                log::warn!("I've stopped all PWM outputs,");
                log::warn!("and I've stopped accepting commands.");
                log::warn!("Reset your system to start over.");
                log::warn!("------------------------------------");

                led.set_high().unwrap();
                loop {
                    systick.delay(1_000);
                    cortex_m::asm::wfe();
                }
            }
            // Parser detected an error
            Err(err) => {
                log::warn!("{:?}", err);
            }
        };
    }
}

/// The minimum duty cycle for the ESC PWM protocol is 50% duty cycle.
/// Since the underlying PWM duty cycle spans all `u16` values, the minimum
/// duty cycle is half of that.
const MINIMUM_DUTY_CYCLE: u16 = u16::max_value() >> 1;

/// Converts a percentage to a 16-bit duty cycle that implements the ESC PWM protocol
fn percent_to_duty(pct: f64) -> u16 {
    ((MINIMUM_DUTY_CYCLE as f64) * (pct / 100.0f64)) as u16 + MINIMUM_DUTY_CYCLE
}

/// Compute the rate at which we should blink the LED based on the
/// PWM duty cycles. Defines a duration based on the highest PWM
/// duty cycle.
fn pwm_to_blink_period(esc: &dyn ESC<Motor = QuadMotor>) -> Duration {
    const SLOWEST_BLINK_NS: i64 = 2_000_000_000;
    const FASTEST_BLINK_NS: i64 = SLOWEST_BLINK_NS / 100;
    const MOTORS: [QuadMotor; 4] = [QuadMotor::A, QuadMotor::B, QuadMotor::C, QuadMotor::D];
    if let Some(duty) = MOTORS
        .iter()
        .map(|motor| percent_to_duty(100f64 * esc.throttle(*motor) as f64))
        .max()
    {
        let ns = (duty as i64) * (FASTEST_BLINK_NS - SLOWEST_BLINK_NS) / 0xFFFF + SLOWEST_BLINK_NS;
        Duration::from_nanos(ns as u64)
    } else {
        Duration::from_nanos(SLOWEST_BLINK_NS as u64)
    }
}
