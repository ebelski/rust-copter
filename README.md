# rust-copter

A quadcopter build using a semi-realtime Teensy running Rust as a flight controller. This will then interface with a Raspberry Pi to run operations that are less time senstive than control loops. The capability to fly autonomously is desired however the RPi will be able to talk to a remote control in the form of another RPi/tablet.

Initially the first step in this project is to have a Rust ecosystem developed to control the Teensy chipset. [mciantyre](https://github.com/mciantyre) developed some cargo to boot a Teensy and control its various functions. That repository can be found [here](https://github.com/mciantyre/teensy4-rs). 

Now that we have some code developed to control the Teensy, some prototyping is necessary to interface sensors and motors to the Teensy. The IMU chip for navigation we're using is the MPU-9250. This has well developed documentation and a nice dev board available from [Sparkfun](https://www.sparkfun.com/products/13762) A demo in Rust is implemented to access the IMU readings from the Teensy. The accelerometer, rate gyro, and magnetometer on the chip all have different bandwidths, so they must all be queried separately. The accelerometer can be queried at 4 kHz, the rate gyro can be queried at 8000 Hz, and the magnetometer can be queried at 8 Hz. The [demo program](https://github.com/e-belski/rust-copter/tree/master/demos/mpu9250-getting-started) shows how to sample the accelerometer on the MPU chip.

More documentation to come as we develop it.

## Getting started

[Install `git-lfs`](https://git-lfs.github.com). We use `git-lfs` to manage larger files, like PDF reference manuals and 3D models, in this repository.

The Rust libraries and applications in this project are expected to run in a Teensy 4. To build the code, install the [dependencies](https://github.com/mciantyre/teensy4-rs#dependencies) required by the `teensy4-rs` project.

Once you've installed all of the dependencies, check-out this repository, then try

```
cargo build --target thumbv7em-none-eabihf
```

to build the code.

## Downloading software to a Teensy 4

Use the [`task.py` script](task.py) to build a demo that can run on a Teensy 4. For example,

```
python task.py demo pwm-control
```

will build a `pwm-control` demo binary, ready to be installed on your Teensy 4. `task.py` prints the location of the final program, so take note of the output. Use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html) to download the program to your Teensy 4. If the [`teensy_loader_cli` application](https://github.com/PaulStoffregen/teensy_loader_cli) is available on your `PATH`, `task.py` will run it, prompting you to download the program to your Teensy 4.
