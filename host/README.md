# Host-side tools

The `host` directory contains libraries and programs for host-side software.

Unless otherwise stated, all commands below should be run from the root of the
repository. The guides below also assume that you've installed thedependencies
in the [top-level README](../README.md).

## `pymotion-sensor`

A Python library to parse motion sensor readings. The library exposes the
deserialization routine for types in the `motion-sensor` crate. The
library lets you deserialize raw accelerometer, gyroscope, and magnetometer
readings using Python.

The library is written in Rust. We recommend using
[maturin](https://github.com/PyO3/maturin) to build and install the library.

```bash
$ pip install 'maturin>=0.9,<0.10'
$ maturin build --release --manifest-path host/pymotion-sensor/Cargo.toml
```

The tool will note the path to the built wheel. Use `pip` to install the
wheel. Then, ensure that the library installation worked:

```python
>>> import motion_sensor
>>> help(motion_sensor)
```

For an example of a Python tool that uses `pymotion-sensor`, see
`imu-parse.py`.

## pypwm_control

A pure Python package that lets you interface the `pwm-control` demo on the
embedded system. This package depends on the `pymotion-sensor` library. Use
the package to

- stream IMU readings
- control PWM outputs that drive the ESC

See `imu-parse.py` for an example of IMU streaming. See `esc-throttle.py` for
an example of ESC control.

## `imu-parse.py`

A simple Python program that demonstrates how to parse motion sensor readings
from a serial port. The motion sensor readings may be published by one of the
demos in this repository, such as the `pwm-control` demo. The script binds to
the provided serial port, and deserializes the accelerometer, gyroscope, and
magnetometer readings.

To use the script, install the `pymotion-sensor` library above. Then, install
the additional requirements:

```bash
$ pip install -r host/requirements.text
```

Finally, run the script, supplying your serial port:

```bash
$ python host/imu-parse.py --help
$ python host/imu-parse.py COMx          # Windows
$ python host/imu-parse.py /dev/tty.USBx # *nix
```

## `esc-throttle.py`

A simple Python program for controlling the `pwm-control` PWM outputs. See the
`pwm-control` documentation for more information on the available motors.

To use the script, install the `pymotion-sensor` library above. Then, install
the additional requirements:

```bash
$ pip install -r host/requirements.text
```

Finally, run the script, supplying

- your serial port
- the motor of interest
- the motor throttle

```bash
# Set motor A to 72% throttle (Windows)
$ python host/esc-throttle.py COMx A 72
```
