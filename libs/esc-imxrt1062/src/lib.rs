//! ESC implementation for the i.MX RT's PWM driver
//!
//! The implementation is *very* tightly-coupled to two specific PWM modules
//! and submodules. It's hard for us to generalize the implementation due to
//! design decisions in the `imxrt_hal` crate. We should fix this at a later
//! time.
//!
//! If there's a need to change the four PWM pins, you'll need to change the
//! `XMod`, `YMod`, `XSub`, and `YSub` types inside of this crate, then recompile.
//!
//! (We could generalize over the `XMod` and `YMod` types. But there's little point, since
//! users still can't generalize over the *submodules*. This is a limitation imposed by
//! the `imxrt-hal` crate.)

// Ian's notes about why this isn't the best...
//
// The `imxrt-hal` crate uses macros to define the `Pwm` implementations on the
// `Controller` type. That would require us to also use macros here as a means
// of generalization. That's annoying.
//
// Also, we can't use immutable `Pwm` methods without first having a mutable `Pins`
// refernce. That's annoying, but not as annoying; we can use a `RefCell` to work
// around that. It should still be fixed!

#![no_std]

use esc::{QuadMotor, ESC};

use imxrt_hal::{
    iomuxc::pwm::Pin,
    pwm::{module, output, submodule, Channel, Handle, Pins},
};

use embedded_hal::Pwm;

use core::{cell::RefCell, time::Duration};

pub type XMod = module::_1;
type XSub = submodule::_3;

pub type YMod = module::_2;
type YSub = submodule::_2;

pub struct Module<A, B, C, D> {
    handle_ab: Handle<XMod>,
    handle_cd: Handle<YMod>,
    pins_ab: Pins<A, B>,
    pins_cd: Pins<C, D>,
}

impl<A, B, C, D> Module<A, B, C, D>
where
    A: Pin<Module = XMod, Output = output::A, Submodule = XSub>,
    B: Pin<Module = XMod, Output = output::B, Submodule = <A as Pin>::Submodule>,
    C: Pin<Module = YMod, Output = output::A, Submodule = YSub>,
    D: Pin<Module = YMod, Output = output::B, Submodule = <C as Pin>::Submodule>,
{
    pub fn new(
        mut handle_ab: Handle<XMod>,
        mut pins_ab: Pins<A, B>,
        mut handle_cd: Handle<YMod>,
        mut pins_cd: Pins<C, D>,
        period: Duration,
    ) -> Self {
        let mut ab = pins_ab.control(&mut handle_ab);
        ab.set_period(period);
        ab.enable(Channel::A);
        ab.enable(Channel::B);

        let mut cd = pins_cd.control(&mut handle_cd);
        cd.set_period(period);
        cd.enable(Channel::A);
        cd.enable(Channel::B);

        let mut module = Module {
            handle_ab,
            pins_ab,
            handle_cd,
            pins_cd,
        };

        for motor in &[QuadMotor::A, QuadMotor::B, QuadMotor::C, QuadMotor::D] {
            module.set_duty(*motor, MINIMUM_DUTY_CYCLE)
        }

        module
    }

    fn set_duty(&mut self, motor: QuadMotor, duty: u16) {
        match motor {
            QuadMotor::A => {
                let mut ctrl = self.pins_ab.control(&mut self.handle_ab);
                ctrl.set_duty(Channel::A, duty);
            }
            QuadMotor::B => {
                let mut ctrl = self.pins_ab.control(&mut self.handle_ab);
                ctrl.set_duty(Channel::B, duty);
            }
            QuadMotor::C => {
                let mut ctrl = self.pins_cd.control(&mut self.handle_cd);
                ctrl.set_duty(Channel::A, duty);
            }
            QuadMotor::D => {
                let mut ctrl = self.pins_cd.control(&mut self.handle_cd);
                ctrl.set_duty(Channel::B, duty);
            }
        }
    }

    /// This needs to be `&mut self`, because `control()` takes a mutable
    /// receiver. See notes above about `imxrt_hal` crate limitations.
    fn get_duty(&mut self, motor: QuadMotor) -> u16 {
        match motor {
            QuadMotor::A => {
                let ctrl = self.pins_ab.control(&mut self.handle_ab);
                ctrl.get_duty(Channel::A)
            }
            QuadMotor::B => {
                let ctrl = self.pins_ab.control(&mut self.handle_ab);
                ctrl.get_duty(Channel::B)
            }
            QuadMotor::C => {
                let ctrl = self.pins_cd.control(&mut self.handle_cd);
                ctrl.get_duty(Channel::A)
            }
            QuadMotor::D => {
                let ctrl = self.pins_cd.control(&mut self.handle_cd);
                ctrl.get_duty(Channel::B)
            }
        }
    }

    fn kill(&mut self) {
        for motor in &[QuadMotor::A, QuadMotor::B, QuadMotor::C, QuadMotor::D] {
            self.set_duty(*motor, 0)
        }
    }
}

/// Implements the "slow" ESC protocol
///
/// - 2ms PWM period
/// - 0% throttle => 50% PWM duty cycle
/// - 100% throttle => 100% PWM duty cycle
pub struct Slow<A, B, C, D>(RefCell<Module<A, B, C, D>>);

impl<A, B, C, D> Slow<A, B, C, D>
where
    A: Pin<Module = XMod, Output = output::A, Submodule = XSub>,
    B: Pin<Module = XMod, Output = output::B, Submodule = <A as Pin>::Submodule>,
    C: Pin<Module = YMod, Output = output::A, Submodule = YSub>,
    D: Pin<Module = YMod, Output = output::B, Submodule = <C as Pin>::Submodule>,
{
    pub fn new(
        handle_ab: Handle<XMod>,
        pins_ab: Pins<A, B>,
        handle_cd: Handle<YMod>,
        pins_cd: Pins<C, D>,
    ) -> Self {
        Slow(RefCell::new(Module::new(
            handle_ab,
            pins_ab,
            handle_cd,
            pins_cd,
            Duration::from_micros(2000),
        )))
    }

    /// Kill all PWM outputs
    pub fn kill(&mut self) {
        self.0.get_mut().kill();
    }
}

/// The minimum duty cycle for the ESC PWM protocol is 50% duty cycle.
/// Since the underlying PWM duty cycle spans all `u16` values, the minimum
/// duty cycle is half of that.
const MINIMUM_DUTY_CYCLE: u16 = u16::max_value() >> 1;

/// Converts a percentage to a 16-bit duty cycle that implements the ESC PWM protocol
fn percent_to_duty(pct: f32) -> u16 {
    ((MINIMUM_DUTY_CYCLE as f32) * pct) as u16 + MINIMUM_DUTY_CYCLE
}

/// Converts a 16-bit duty cycle that implements the ESC PWM protocol to a percentage
fn duty_to_percent(duty: u16) -> f32 {
    (duty.saturating_sub(MINIMUM_DUTY_CYCLE) as f32) / (MINIMUM_DUTY_CYCLE as f32)
}

impl<A, B, C, D> ESC for Slow<A, B, C, D>
where
    A: Pin<Module = XMod, Output = output::A, Submodule = XSub>,
    B: Pin<Module = XMod, Output = output::B, Submodule = <A as Pin>::Submodule>,
    C: Pin<Module = YMod, Output = output::A, Submodule = YSub>,
    D: Pin<Module = YMod, Output = output::B, Submodule = <C as Pin>::Submodule>,
{
    type Motor = QuadMotor;

    fn throttle(&self, motor: Self::Motor) -> f32 {
        let mut this = self.0.borrow_mut();
        duty_to_percent(this.get_duty(motor))
    }

    fn set_throttle(&mut self, motor: Self::Motor, percent: f32) {
        let percent = if percent < 0.0 {
            0.0
        } else if percent >= 1.0 {
            1.0
        } else {
            percent
        };
        self.0.get_mut().set_duty(motor, percent_to_duty(percent))
    }
}