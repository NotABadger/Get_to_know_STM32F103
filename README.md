# Get_to_know_STM32F103
Working on the STM32 platform at work, would like to explore the lower levels which the almighty grey beards before me already implemented.
p.s. This is what nerds do for fun in weekends...

## Plan of attack
Going to repeat the whole college course with going through the chip registers and building a HAL.
* Turn LED on at start of program
* Read input pin from button, turn LED on when button is pressed
* Make a timer work so we can have a delay
    * Implement Blink
* Use timer(s) for time stamp
* Use timer for interrupt
    * Implement Blink on interrupt
* Implement PWM
    * Implement glowing LED
* Use ADC for analog value
  * Read POT meter and write to LED on above certain value

## Technology used
Although this is written in Rust, it will not follow the Rust principles of ownership or embedded-hal crate. 
This is done because we want to get to know the chip registers, not the whole Rust architecture.

## Resources for learning
- ([Rust book](https://doc.rust-lang.org/book/title-page.html))
- ([Rust programming by example](https://doc.rust-lang.org/rust-by-example/index.html))
- ([Rust cargo book](https://doc.rust-lang.org/cargo/reference/workspaces.html))
- ([Rustonomicon book](https://doc.rust-lang.org/nomicon/intro.html))
- ([Rust Embedded book](https://doc.rust-lang.org/stable/embedded-book/intro/index.html))
- ([Cortex-m crate](https://crates.io/crates/cortex-m/))
- ([Cortex-m-rt crate](https://crates.io/crates/cortex-m-rt/))

## How to build
- ([Install Rust](https://www.rust-lang.org/tools/install))
- install openOCD
- ([install gdb multi arch, and set udev rules](https://doc.rust-lang.org/stable/embedded-book/intro/install/linux.html?search=))
- ([add "thumbv7m-none-eabi" target](https://doc.rust-lang.org/stable/embedded-book/start/qemu.html))

