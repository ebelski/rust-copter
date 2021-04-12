#!/usr/bin/env python3

"""IMU data stream

The implementation uses the motion_sensor library to decode IMU readings.
"""

import motion_sensor


class ImuReading:
    """Valid IMU reading types"""

    Acc = motion_sensor.Acc
    Gyro = motion_sensor.Gyro
    Mag = motion_sensor.Mag


class ImuStream:
    """An iterator over the IMU readings"""

    def __init__(self, serial):
        """Construct a stream of IMU readings that are decoded
        from a serial device

        By default, ImuStream enables all readings. Use disable_readings
        to change the behavior.

        The user is responsible for configuring the serial port.
        """
        self._serial = serial
        self._measurement_filter = {
            ImuReading.Acc,
            ImuReading.Gyro,
            ImuReading.Mag,
        }

    def _prime_readings(self):
        MAX_RETRIES = 1000

        retries = 0
        while retries < MAX_RETRIES:
            self._serial.reset_input_buffer()
            buffer = bytearray(self._serial.read_until(b"\0"))
            try:
                motion_sensor.convert_readings(buffer)
                return
            except ValueError:
                retries += 1

        raise TimeoutError(f"Could not find prime readings after {MAX_RETRIES} retries")

    def disable_readings(self, *reading_types):
        """Prevent the IMU from streaming certain types of readings

        reading_types is a collection of types. See ImuReading for the
        valid types.

        The implementation could filter the readings from the host, or command
        the embedded system to not poll for that data.
        """
        for reading_type in reading_types:
            self._measurement_filter.remove(reading_type)

    def enable_readings(self, *reading_types):
        """Tell the IMU to stream certain types of readings

        reading_types is a collection of types. See ImuReading for the
        valid types.

        This could make a call to the embedded system to poll for the new data,
        or it could change a filter on the host.
        """
        for reading_type in reading_types:
            self._measurement_filter.add(reading_type)

    def stream(self):
        """Stream IMU readings with an endless generator

        Pass the output of stream() to a for loop, or type that accepts an
        iterator. The generator blocks indefinitely. Raises a TimeoutError
        if the implementation could not prime the IMU data stream.
        """
        self._prime_readings()
        while True:
            buffer = bytearray(self._serial.read_until(b"\0"))
            readings = motion_sensor.convert_readings(buffer)
            for reading in readings:
                if type(reading) in self._measurement_filter:
                    yield reading
