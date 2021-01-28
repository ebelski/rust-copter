# Getting Started - Software

This document describes how to install all software tools for a brand-new
development environment. It assumes that you're familiar with a command-line
interface for your development system (Linux, macOS, or Windows).

This document may also help if you have an existing environment, and you want
to make sure it has all of the project dependencies. If this document conflicts
with the documentation or references it points to, trust the other reference,
and let us know about the discrepancy.

## TL;DR

If you're on Linux or macOS, run `first-time-setup.sh` from a Bash-like shell:

```bash
$ cd docs
$ ./first-time-setup.sh
```

If you'd like to help us automate the Windows setup, let us know!

## Step by Step

The step-by-step instructions tell you how to manually install all software
and dependencies. The instructions should work for most Linux, macOS, and
Windows development systems. Follow these steps in order.

### Install Rust

Much of the project's embedded system software is written in Rust. Install Rust
by following the instructions in [The Rust Programming Language][the-book-ch1]
book. We assume that your installation will include `rustup` for managing Rust
toolchains.

[the-book-ch1]: https://doc.rust-lang.org/book/ch01-01-installation.html

Make sure that you have all additional dependencies. If you're on Linux /macOS,
you might need to install a linker. If you use Windows, you may need to install
the Build Tools for Visual Studio 2019. Read the relevant sections of
[The Rust Programming Language][the-book-ch1] (section 1.1) for more
information.

**Success**: All of these commands succeed:

```bash
$ rustup --version
$ rustc --version
$ cargo --version
```

### Install embedded targets

The embedded system is built on an ARMv7 microcontroller. Install the following
Rust target to cross-compile code for the embedded system:

```bash
$ rustup target add thumbv7em-none-eabihf
```

**Success**: Run `rustup target list`, and show that `thumbv7em-none-eabihf` is
identified as "installed."

### Install llvm-tools-preview

Once you've installed Rust, run the following command to install
`llvm-tools-preview`. We need these tools for the next tool.

```bash
$ rustup component add llvm-tools-preview
```

**Success**: This command succeeds:

```bash
$ rust-objcopy --version
```

### Install cargo binutils

We use `cargo binutils` throughout our build system to simplify program
creation for the embedded system. Run the following command to install
`cargo binutils`

```bash
$ cargo install cargo-binutils
```

**Success**: This command succeeds:

```bash
$ cargo objcopy --version
```

To learn more about `cargo binutils`, see [here][cargo-binutils].

[cargo-binutils]: https://github.com/rust-embedded/cargo-binutils

### Install Python

We use Python for cross-platform scripting, and for tooling and testing on our
host systems. Make sure that your system has at least a Python 3.7 installation.
For more information, follow the Python installation instructions for your
system.

**Success**: You can run `python` (or `python3`) from the command line. The
version is at least Python 3.7.

```bash
$ python --version
```

### Install a Teensy Loader

Our first embedded system uses a Teensy 4.0. In order to upload programs to the
Teensy 4, we need a Teensy Loader program. Either

- install the graphical [Teensy Loader Application]; or
- install / build the [`teensy_loader_cli`]

The Teensy Loader Application should be available with the Teensyduino add-ons
for Arduino.

[`teensy_loader_cli`]: https://github.com/PaulStoffregen/teensy_loader_cli
[Teensy Loader Application]: https://www.pjrc.com/teensy/loader.html

Some of our automation assumes that you have the [`teensy_loader_cli`]
available. But, that's only for convenience. You may use the graphical
application to load your programs.

### Next Steps

By this point, you have the base software installation for embedded and
development tools. But, there may be other, tool-specific dependencies
that are not covered by this documentation. Consult each tool's
documentation for more information.
