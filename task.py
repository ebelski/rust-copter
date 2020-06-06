#!/usr/bin/env python3

"""
Run arbitrary tasks in the repo

- Ask for help

    ./task.py --help

  Ask for help on a specific task, like the `demo` task:

    ./task.py demo --help

- Build and flash a demo, like the `pwm-control` demo:

    ./task.py demo pwm-control

  If the command-line Teensy flashing tool isn't available, `demo` will print
  the location of the hex file on the command-line. If it is available, `demo`
  will call the flashing tool to load the program onto the Teensy.
"""

import argparse
import logging
import os
import pathlib
import subprocess
import shutil

RUSTFLAGS = "-C link-arg=-Tlink.x"
TARGET = "thumbv7em-none-eabihf"
OBJCOPY = "arm-none-eabi-objcopy"
TEENSY_LOADER = "teensy_loader_cli"


def _flash(program: pathlib.Path) -> bool:
    """Calls the command-line Teensy loader to flash the program.
    
    Returns True if the flashing succeeded, or False if there was
    a flashing error.
    """

    if shutil.which(TEENSY_LOADER):
        cmd = f"{TEENSY_LOADER} --mcu=TEENSY40 -w -v {program}"
        logging.debug("Found %s, running '%s'", TEENSY_LOADER, cmd)
        subprocess.run(cmd, shell=True, check=True)
        return True

    else:
        logging.debug("No %s application found", TEENSY_LOADER)
        return False


def _bin2hex(binary: pathlib.Path) -> pathlib.Path:
    """Converts the binary to a hex file
    """

    hex_file = binary.with_suffix(".hex")
    cmd = f"{OBJCOPY} -O ihex -R .eeprom {binary} {hex_file}"
    logging.debug("Running '%s'", cmd)
    subprocess.run(cmd, shell=True, check=True)
    logging.debug("Created hex file at '%s'", hex_file)
    return hex_file


def _cargo_build(crate: str, release: bool) -> pathlib.Path:
    """Run cargo build, building the provided crate

    If `release` is True, build a release build
    """

    mode = ""
    if release:
        mode = "--release"

    env = os.environ.copy()
    env["RUSTFLAGS"] = RUSTFLAGS
    logging.debug("Extended environment with RUSTFLAGS='%s'", RUSTFLAGS)

    cmd = f"cargo build --target {TARGET} {mode} --package {crate}"
    logging.debug("Running '%s'", cmd)
    subprocess.run(cmd, shell=True, check=True, env=env)

    return (
        pathlib.Path("target")
        / TARGET
        / ("release" if args.release else "debug")
        / crate
    )


def demo(args):
    """Handler for the `demo` task
    """

    crate = args.crate
    if not crate.startswith("demo-"):
        logging.debug("User did not include 'demo-' prefix, so adding it now...")
        crate = f"demo-{crate}"

    logging.debug("Using demo crate '%s'", crate)

    target = _cargo_build(crate, args.release)
    hex_file = _bin2hex(target)
    if not args.flash or not _flash(hex_file):
        print(str(hex_file))


parser = argparse.ArgumentParser(description="rust-copter task runner")
parser.add_argument("-v", "--verbose", help="verbose logging", action="store_true")
subparsers = parser.add_subparsers(title="tasks", description="valid tasks",)

parser_demo = subparsers.add_parser("demo", help="build and deploy a demo")
parser_demo.add_argument("crate", help="the demo crate name")
parser_demo.add_argument(
    "-d",
    "--debug",
    help="build a debug version (defaults to release)",
    action="store_false",
    dest="release",
)
parser_demo.add_argument(
    "--skip-flash",
    help="skip the (optional) flashing step",
    action="store_false",
    dest="flash",
)
parser_demo.set_defaults(func=demo)

args = parser.parse_args()
if args.verbose:
    logging.basicConfig(level=logging.DEBUG)
args.func(args)
