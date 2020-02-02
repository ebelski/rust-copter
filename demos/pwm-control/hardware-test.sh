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
cp target/thumbv7em-none-eabihf/$BUILD_MODE/pwm_control out/pwm_control
# arm-none-eabi-objdump -d -S -C out/pwm_control > out/pwm_control.lst
# arm-none-eabi-objdump -t -C out/pwm_control > out/pwm_control.sym
arm-none-eabi-objcopy -O ihex -R .eeprom out/pwm_control out/pwm_control.hex

if [ -x "$(command -v teensy_loader_cli)" ]; then
    teensy_loader_cli --mcu=TEENSY40 -w -v out/pwm_control.hex
fi