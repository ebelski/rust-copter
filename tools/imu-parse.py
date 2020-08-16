#!/usr/bin/env python3

"""Parse IMU data from a serial connection
"""

import argparse

import motion_sensor
import serial

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument(
    "port", type=str, help="Serial port (COMx on Windows, or /dev/tty.USBx on *nix)"
)

DEFAULT_BAUD = 115_200
parser.add_argument(
    "--baud", type=int, help=f"Baud rate (default {DEFAULT_BAUD})", default=DEFAULT_BAUD
)

args = parser.parse_args()
with serial.Serial(args.port, args.baud) as ser:
    ser.reset_input_buffer()
    while True:
        buffer = bytearray(ser.read_until(b"\0"))
        readings = motion_sensor.convert_readings(buffer)
        for reading in readings:
            print(f"{type(reading)}: ({reading.x}, {reading.y}, {reading.z})")
