# rust-copter

A quadcopter build using a semi-realtime Teensy 4.0 running Rust ([mciantyre](https://github.com/mciantyre) developed [this](https://github.com/mciantyre/teensy4-rs) to run Rust on a Teensy 4.0).

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
