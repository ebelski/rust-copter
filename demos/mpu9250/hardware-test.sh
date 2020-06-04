#!/usr/bin/env bash

set -e

BUILD_MODE_FLAG="--release"
BUILD_MODE="release"

if [ "$1" = "--debug" ]; then
    BUILD_MODE_FLAG=""
    BUILD_MODE="debug"
elif [ "$1" != "" ]; then 
    echo "Unknown option '$1'"
    exit 1
fi

cargo build $BUILD_MODE_FLAG
rm -Rf out
mkdir out
cp target/thumbv7em-none-eabihf/$BUILD_MODE/teensy4-rs-mpu9250 out/teensy4-rs-mpu9250
arm-none-eabi-objdump -d -S -C out/teensy4-rs-mpu9250 > out/teensy4-rs-mpu9250.lst
arm-none-eabi-objdump -t -C out/teensy4-rs-mpu9250 > out/teensy4-rs-mpu9250.sym
arm-none-eabi-objcopy -O ihex -R .eeprom out/teensy4-rs-mpu9250 out/teensy4-rs-mpu9250.hex

if [ -x "$(command -v teensy_loader_cli)" ]; then
    teensy_loader_cli --mcu=TEENSY40 -w -v out/teensy4-rs-mpu9250.hex
fi