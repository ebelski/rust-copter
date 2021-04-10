#!/usr/bin/env python3

"""Parse IMU data from a serial connection
"""

import argparse

from pypwm_control import ImuStream, ImuReading
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
    imu_stream = ImuStream(ser)

    # Disable accelerometer and magnetomter readings...
    imu_stream.disable_readings(ImuReading.Acc, ImuReading.Mag)
    # ... and re-enable the accelerometer
    imu_stream.enable_readings(ImuReading.Acc)

    for reading in imu_stream.stream():
        print(f"{type(reading)}: ({reading.x}, {reading.y}, {reading.z})")
