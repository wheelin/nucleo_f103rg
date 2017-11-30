# Nucleo F103RG board support package written in Rust

This repo contains code for a learning project. It isn't a full blown generic hardware abstraction layer and it may
certainly contain aweful contructs and wrong patterns. Please, open an issue if you have some useful advices to tell me.

## Goals
The repo's goal is to cover most of the functionalities of the microcontroller as a learning project for Rustlang.

* Clock tree settings
* Low power modes
* Systick as a delay and time monitor
* GPIO
* Timers with functions registration for timeouts
* PWM output
* Input capture
* ADC
* I2C  
* SPI
* USART
* Serial port to computer
* RTC with calendar functions and alarm registration (demo only, will not work because of 32k crystal miss)
* DMA
* SD card

A small round-robin scheduler is planned as well. Will be linked here when some work has been done.

## Non goals
The following parts will require to much work for me at the moment. May be covered in the future, or not.
* USB
* CAN

## Already working
* Clock tree settings : hardfault when sysclk clocked above ~60MHz. Otherwise works.
* GPIO : can read, write, configure as analog, alternate function, can configure slew rate, pull-up/down and push-pull functionalities. Working on functions interrupt registration.
* Serial : formatted writting is working. May not work if the baudrate is too high (Problems with baudrate calculation). Next steps are binary write/receive for custom protocol design.
* SysTick : used to create delays, with ms as default resolution

At the moment, this code can blink a led repeatedly, print formatted text in a serial terminal, i.e. `screen /dev/ttyACMx 9600` and read the on-board button's state.

## Next steps
* Create an example file for each finished part
* Cover gpio input interrupt as a proof of concept for the other interrupt based parts.
