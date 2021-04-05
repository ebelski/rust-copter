# Tools

The `tools` directory contains libraries and programs for host-side software.

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
$ maturin build --release --manifest-path tools/pymotion-sensor/Cargo.toml
```

The tool will note the path to the built wheel. Use `pip` to install the
wheel. Then, ensure that the library installation worked:

```python
>>> import motion_sensor
>>> help(motion_sensor)
```

For an example of a Python tool that uses `pymotion-sensor`, see
`imu-parse.py`.

## `imu-parse.py`

A simple Python program that demonstrates how to parse motion sensor readings
from a serial port. The motion sensor readings may be published by one of the
demos in this repository, such as the `pwm-control` demo. The script binds to
the provided serial port, and deserializes the accelerometer, gyroscope, and
magnetometer readings.

To use the script, install the `pymotion-sensor` library above. Then, install
the additional requirements:

```bash
$ pip install -r tools/requirements.text
```

Finally, run the script, supplying your serial port:

```bash
$ python tools/imu-parse.py --help
$ python tools/imu-parse.py COMx          # Windows
$ python tools/imu-parse.py /dev/tty.USBx # *nix
```
