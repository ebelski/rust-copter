#!/usr/bin/env python3

"""
PwmControl module, which supports the embedded system's motor control
protocol. See the pwm-control example docs for details.

Usage:

    my_serial = # Open your serial interface...
    pwm_control = PwmControl(my_serial)

    # Set motor "A" to 37% throttle
    pwm_control.motor("A").set_throttle(37)

Notes:

    Motor constructor should be considered private. Do not use.
    Use PwmControl methods to acquire the motor objects.
"""


class Motor:
    """A handle to a single motor"""

    def __init__(self, motor_id: str, serial):
        self._motor_id = motor_id
        self._serial = serial

    def _encode(self, pct: float):
        return bytes(f"{self._motor_id}.{pct}\r".encode("ASCII"))

    def set_throttle(self, pct: float):
        """Set the motor throttle as a percentage between 0 and 100

        Throws a ValueError if the percentage is not a valid value.
        May also throw an error when writing to the serial device.

        Returns Nothing on success.
        """

        if 0.0 <= pct <= 100:
            cmd = self._encode(pct)
            self._serial.write(cmd)
        else:
            raise ValueError(f"Invalid motor throttle percentage: {pct}")


class PwmControl:
    """The PWM motors that interface the pwm-control's ESC"""

    def __init__(self, serial):
        self._serial = serial

    def motor(self, motor_id: str):
        """Get a handle to the motor identified by the motor_id string

        Returns nothing is the motor ID is invalid. See the pwm-control
        documentation to understand valid motor IDs.
        """
        if motor_id in {"A", "B", "C", "D"}:
            return Motor(motor_id, self._serial)

    def reset(self):
        """Reset all throttles to zero percent

        This will be more efficient than setting 0% throttle on all
        the motors yourself.
        """
        self._serial.write(b" ")

    def kill(self):
        """Invoke the kill switch on the embedded system

        This stops all PWM output, and prevents the system from responding
        to any throttle commands. You should stop using this object after
        invoking kill().
        """
        self._serial.write(b"\\")
