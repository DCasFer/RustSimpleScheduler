
use embrs::arm_m::sys_tick;
use embrs::stm32f4::gpio::{self, gpioa};
use embrs::stm32f4::rcc::{self, RCC, AhbPeripheral, ApbPeripheral};

const TOGGLE_HZ : u32 = 10;
const HZ : u32 = 160_000_000;


/* Function name           : CLOCKS                   */
/* Description             : Constant struct that is used for UART and UART is used by the LEDs.
                            It is also used byt he systick interrupt*/
pub const CLOCKS : rcc::ClockConfig = rcc::ClockConfig {
    crystal_hz: 8_000_000_f32,
    crystal_divisor: 4,
    vco_multiplier: 160,
    general_divisor: rcc::SysPrescaler::Div2,
    pll48_divisor: 4,

    ahb_divisor: None,
    apb1_divisor: Some(rcc::ApbPrescaler::Div4),
    apb2_divisor: Some(rcc::ApbPrescaler::Div2),

    flash_latency: 5,
};


/* Function name           : inti_SYSTICK()                    */
/* Description             : Sets up the SYSTICK frecuency,  clocks and enables it    */
pub fn init_SYSTICK (){
    let cycles_per_toggle = HZ / TOGGLE_HZ;
    sys_tick::SYS_TICK.write_rvr(cycles_per_toggle - 1);
    sys_tick::SYS_TICK.write_csr(
        sys_tick::SYS_TICK.read_csr()
        .with_enable(true)
        .with_tickint(true)
        .with_clksource(sys_tick::ClkSource::ProcessorClock));

}

/* Function name           : init_uart()                    */
/* Description             : Enables and setup the UART which will be used by the LEDs    */
pub fn init_uart() {
    use embrs::stm32f4::usart::*;

    // Enable clock to USART2.
    RCC.enable_clock(ApbPeripheral::Usart2);

    USART2.update_cr1(|v| v.with_ue(true));

    let speeds = CLOCKS.compute_speeds();

    let clk = speeds.get_clock_for(ApbPeripheral::Usart2);
    let brr = (clk / 115200_f32 + 0.5) as u32;

    USART2.update_brr(|v| v.with_mantissa(brr >> 4)
                      .with_fraction(brr & 0xF));

    USART2.update_cr1(|v| v.with_te(true));

    RCC.enable_clock(AhbPeripheral::GpioA);
    // Configure its TX pin (PA2) as AF7
    gpioa().set_alternate_function(gpio::P2, gpio::Function::AF7);
    gpioa().set_mode(gpio::P2, gpio::Mode::Alternate);
}
