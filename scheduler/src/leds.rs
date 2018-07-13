
use embrs::stm32f4::gpio::{self, gpiod};
use embrs::stm32f4::rcc::{ RCC, AhbPeripheral};



/* Function name           : led_pin_12()                    */
/* Description             : Returns the P12 from the static GPIO struct    */
pub fn led_pin_12() -> gpio::PinMask {
    gpio::P12
}

/* Function name           : led_pin_14()                    */
/* Description             : Returns the P14 from the static GPIO struct    */
pub fn led_pin_14() -> gpio::PinMask {
    gpio::P14
}

/* Function name           : led_pins()                    */
/* Description             : Returns the P12 and _14 from the static GPIO struct to
                            flash two LEDs together*/
pub fn led_pins () -> gpio::PinMask {
    led_pin_12() | led_pin_14()
}

/* Function name           : led_leds()                    */
/* Description             : Initializes the LEDs clock and modes    */
pub fn init_leds() {
    // Enable clock to GPIOD so we can mess with its registers.
    RCC.enable_clock(AhbPeripheral::GpioD);

    // Configure our pins for push-pull digital output.
    gpiod().set_mode(led_pins(), gpio::Mode::Gpio);
    gpiod().set_output_type(led_pins(), gpio::OutputType::PushPull);
}

/* Function name           : task_test()                    */
/* Description             : Used for debugging purposes
                            It flashes two LEDs*/
pub fn task_test (){
    use embrs::stm32f4::usart::*;

    if gpiod().get(led_pins()).is_empty() {
        USART2.send8(b'1');
        gpiod().set(led_pins())
    } else {
        USART2.send8(b'0');
        gpiod().clear(led_pins())
    }
}


/* Function name           : task_1()                    */
/* Description             : The first task taht will be scheduled  */
pub fn task_1 (){
    use embrs::stm32f4::usart::*;
    let mut counter = 0;
    let infinite = true;


    while infinite {
        if (counter > 0) && (counter < 150000) {
            USART2.send8(b'1');
            gpiod().set(led_pin_12());
        }
        else if (counter > 150001) && (counter < 340000) {
            USART2.send8(b'0');
            gpiod().clear(led_pin_12());
        }
        counter = (counter + 1) % 340000;
    }
}

/* Function name           : task_2()                    */
/* Description             : The second task taht will be scheduled  */
pub fn task_2 () {
    use embrs::stm32f4::usart::*;
    let mut counter = 0;
    let infinite = true;

    while infinite {
        if (counter > 0) && (counter < 150000) {
            USART2.send8(b'1');
            gpiod().set(led_pin_14())
        }
        else if (counter > 150001) && (counter < 340000) {
            USART2.send8(b'0');
            gpiod().clear(led_pin_14())
        }
        counter = (counter + 1) % 340000;
    }

}
