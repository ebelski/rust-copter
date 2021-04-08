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


def _prime_readings(ser):
    # Arbitrary big number
    MAX_RETRIES = 1000

    retries = 0
    while retries < MAX_RETRIES:
        ser.reset_input_buffer()
        buffer = bytearray(ser.read_until(b"\0"))
        try:
            motion_sensor.convert_readings(buffer)
            return
        except ValueError:
            retries += 1

    raise TimeoutError(f"Could not find prime readings after {MAX_RETRIES} retries")


args = parser.parse_args()
with serial.Serial(args.port, args.baud) as ser:
    _prime_readings(ser)

    while True:
        buffer = bytearray(ser.read_until(b"\0"))
        readings = motion_sensor.convert_readings(buffer)
        for reading in readings:
            print(f"{type(reading)}: ({reading.x}, {reading.y}, {reading.z})")
