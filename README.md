# rust-copter

A quadcopter build using a semi-realtime Teensy running Rust as a flight controller. This will then interface with a Raspberry Pi to run operations that are less time senstive than control loops. The capability to fly autonomously is desired however the RPi will be able to talk to a remote control in the form of another RPi/tablet.

Initially the first step in this project is to have a Rust ecosystem developed to control the Teensy chipset. [mciantyre](https://github.com/mciantyre) developed some cargo to boot a Teensy and control its various functions. That repository can be found [here](https://github.com/mciantyre/teensy4-rs). 

Now that we have some code developed to control the Teensy, some prototyping is necessary to interface sensors and motors to the Teensy. The IMU chip for navigation we're using is the MPU-9250. This has well developed documentation and a nice dev board available from [Sparkfun](https://www.sparkfun.com/products/13762) A demo in Rust is implemented to access the IMU readings from the Teensy. The accelerometer, rate gyro, and magnetometer on the chip all have different bandwidths, so they must all be queried separately. The accelerometer can be queried at 4 kHz, the rate gyro can be queried at 8000 Hz, and the magnetometer can be queried at 8 Hz. The [demo program](https://github.com/e-belski/rust-copter/tree/master/demos/mpu9250-getting-started) shows how to sample the accelerometer on the MPU chip.

More documentation to come as we develop it.
