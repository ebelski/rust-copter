#!/usr/bin/env python3

"""Demonstrates ESC motor control from the command line

Usage:

Set PWM output D to 99% throttle. Observe that the LED blinks
rapidly.

    python3 host/esc-throttle.py <your COM port> D 99

Reset throttle to zero:

    python3 host/esc-throttle.py <your COM port> D 0
"""

import argparse

from pypwm_control import PwmControl
import serial
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument(
    "port", type=str, help="Serial port (COMx on Windows, or /dev/tty.USBx on *nix)"
)
parser.add_argument(
    "motor",
    type=str,
    help="Motor ID",
)
parser.add_argument("throttle", type=float, help="Throttle %")

DEFAULT_BAUD = 115_200
parser.add_argument(
    "--baud", type=int, help=f"Baud rate (default {DEFAULT_BAUD})", default=DEFAULT_BAUD
)

args = parser.parse_args()
with serial.Serial(args.port, args.baud) as ser:
    pwm_control = PwmControl(ser)
    motor = pwm_control.motor(args.motor)

    if not motor:
        print(f"Invalid motor: {args.motor}", file=sys.stderr)
        sys.exit(1)

    motor.set_throttle(args.throttle)
