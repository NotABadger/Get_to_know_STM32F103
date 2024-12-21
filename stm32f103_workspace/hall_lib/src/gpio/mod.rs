/*
    Idea: To set pin high/or low, we must fir define Input or Output...
    SetPinMode(Pin, Mode)
    ReadPin(Pin) -> bool
    SetPin(Pin, state: bool)

    Datasheet notes: 
    Page 159 & 161,
            GPIO has 2 32-bit CONFIGURATION registers  (GPIOx_CRL, GPIOx_CRH)
            GPIO has 2 32-bit DATA registers  (GPIOx_IDR, GPIOx_ODR) GPIOx_IDR => Read val when input , GPIOx_ODR (PxODR) => Write val when output. 
            GPIO has 1 32-bit SET/RESET registers  (GPIOx_BSRR) -> Facilitates atomic (IRQ)
            GPIO has 1 16-bit RESET registers  (GPIOx_BRR) -> Facilitates atomic operations (IRQ)
            GPIO has 1 32-bit LOCKING registers  (GPIOx_LCKR)
    
    Page 171-174, chapter 9.2
            Contains all registers in table form.
            the "x" in GPIOx_CRL is the range A-G
            Total of 14 registers GPIO(A-G)_CRL, & GPIO(A-G)_CRH -> These configure configuration + Pull up & down
            Total of 7 registers GPIO(A-G)_IDR, 16 bits per register -> For reading current pin value when set as input
            Total of 7 Port output data register (GPIOx_ODR) (x=A..G), 16 bits -> For writing output value
            total of 7 Port output Set/Reset register (GPIOx_BSRR) (x=A..G), first 16 bits are for Setting (0 to 1), and 16-31 are for resetting (1 to 0) in atomic fasion.
            Total of 7 registers for Resetting pins states (GPIOx_BRR) (x=A..G), Setting this register will lock a pin state (usefull for input pins)

    Sequence output pin -> Mode 0 & 1 should be set HIGH (50 MHz switch speed), CNF1 = LOW (unless alternate), CNF0 = HIGH for open Drain or LOW for Push-Pull, 
            use GPIOx_BSRR to set "reset" to LOW, or to "set" to HIGH.
    Sequence input pin -> Mode 0 & 1 should be set LOW, CNF1 = LOW (for floating or analog read), CNF1 = HIGH (for pull-up or pull-down), 
                          When CNF1 is LOW, CNF0 should be LOW for analog read, and HIGH for Input floating.
                          When CNF1 is HIGH, CNF0 should be LOW, PxODR should be set to LOW for pull-down and HIGH for pull-up.

*/