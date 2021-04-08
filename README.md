# rust-copter

A quadcopter build using a semi-realtime Teensy 4.0 running Rust ([mciantyre](https://github.com/mciantyre) developed [this](https://github.com/mciantyre/teensy4-rs) to run Rust on a Teensy 4.0).

## Getting started

[Install `git-lfs`](https://git-lfs.github.com). We use `git-lfs` to manage larger files, like PDF reference manuals and 3D models, in this repository.

To get started with the software, see the [Getting Started - Software](docs/GettingStartedSoftware.md) guide.

## Downloading software to a Teensy 4

Use the [`task.py` script](task.py) to build a demo that can run on a Teensy 4. For example,

```
python task.py demo pwm-control
```

will build a `pwm-control` demo binary, ready to be installed on your Teensy 4. `task.py` prints the location of the final program, so take note of the output. Use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html) to download the program to your Teensy 4. If the [`teensy_loader_cli` application](https://github.com/PaulStoffregen/teensy_loader_cli) is available on your `PATH`, `task.py` will run it, prompting you to download the program to your Teensy 4.

## Structure

| Directory       | Contents                                                            |
| --------------- | ------------------------------------------------------------------- |
| `common/`       | Rust crates that are shared between firmware and host-side software |
| `docs/`         | Getting started guides, references, how-tos, project history        |
| `firmware/`     | Rust crates for the embedded system(s)                              |
| `host/`         | Rust crates, Python utilities for host-side tooling                 |
| `solid-models/` | Mechanical parts for the copter                                     |
