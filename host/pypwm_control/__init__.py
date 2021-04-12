#!/usr/bin/env python3

"""
Python interface for the Rust pwm-control ESC demo software

Use this Python package to

- control the ESC through PWM outputs
- stream IMU sensor readings
"""

from .imu_stream import ImuStream, ImuReading
from .pwm_control import PwmControl

from typing import Tuple


def open(port: str, baud=115_2500) -> Tuple[ImuStream, PwmControl]:
    """Open a serial connection to the embedded system running
    the pwm-control demo. Returns the ImuStream sensor reading
    object, and the PWM control object.

    `port` is a COM port on Windows, or a character device on
    *nix.

    Throws a pyserial error if the serial port is invalid.

    `open` depends on pyserial being installed in your Python
    environment.
    """
    import serial

    device = serial.Serial(port, baud)
    return ImuStream(device), PwmControl(device)
