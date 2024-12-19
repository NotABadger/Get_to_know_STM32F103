# Get_to_know_STM32F103
Working on the STM32 platform at work, would like to explore the lower levels which the almighty grey beards before me already implemented.

## Plan of attack
Going to repeat the whole college course with going through the chip registers and building a HAL.
* Turn LED on at start of program
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
Although this is written in Rust, it will not follow the Rust principles of ownership. 
This is done because I want to get to know the chip registers, not the whole Rust architecture.
