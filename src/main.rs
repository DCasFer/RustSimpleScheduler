// Simplre scheduler for the STM32F4DISCOVERY board.

#![feature(const_fn)]
#![no_std]          //Just Core lib
#![no_main]         //To use the emb.rs startup code. Entry point
#![feature(pointer_methods)]    //Raw pointers

extern crate embrs;
extern crate scheduler;
extern crate rlibc;


use scheduler::leds;
use scheduler::init;
use scheduler::scheduler_core::{self};
use embrs::arm_m::{self, exc};
use embrs::stm32f4::rcc::RCC;
use arm_m::scb::{SCB};
use arm_m::{nvic};



/***************************	Simple task Round Robin Scheduler ********************************/
//--------------Task 1 data
static mut task_stack_1: [u64; scheduler_core::TASK_STACK_SIZE] = [0; scheduler_core::TASK_STACK_SIZE];
//----------- Task 2 data
static mut task_stack_2: [u64; scheduler_core::TASK_STACK_SIZE] = [0; scheduler_core::TASK_STACK_SIZE];


/* Function name           : embrs_main ()                     */
/* Description             : The application entry point.     */
#[no_mangle]
pub extern fn embrs_main() -> ! {

    //Hardware Initialization
    RCC.configure_clocks(&init::CLOCKS);
    leds::init_leds();
    init::init_uart();         //Required in conjunction with configure_clocks
    init::init_SYSTICK();      //Set up SysTick. Required in conjunction with configure_clocks

    //Double Stack alignment.
    SCB.update_ccr(|v| v.with_CCR_STKALIGN_Msk(true));

    //------create_task_frames() and HW32_REG macro for task 1-----
    unsafe {
        scheduler_core::PSP_array[0] = scheduler_core::create_task_frame (task_stack_1.as_mut_ptr() as *mut u32,
                                            leds::task_1);
    }
    //--------------------------------------

    //----- create_task_frames() and HW32_REG macro for task 2-----
    unsafe {
        scheduler_core::PSP_array[1] = scheduler_core::create_task_frame (task_stack_2.as_mut_ptr() as *mut u32,
                                            leds::task_2);
    }
    //----------------------

    unsafe { scheduler_core::curr_task = 0; }   //Switch to task 0 (current task)

    unsafe { scheduler_core::set_PSP(&mut scheduler_core::PSP_array, &scheduler_core::curr_task); } //set PSP to top of the task 1 stack

    nvic::NVIC.enable_irq_raw(scheduler_core::PendSV_IRQ);              //Enable PendSV_IRQ
    nvic::NVIC.set_priority_raw(scheduler_core::PendSV_IRQ, scheduler_core::priority);  //Set PendSV to lowest possible possible priority

    scheduler_core::set_control();

    //ISB()
    arm_m::instruction_synchronization_barrier();

    leds::task_1();

    // Put the processor in an idle state waiting for interrupts from SysTick.
    loop {
        arm_m::wait_for_interrupt();
    }

}
//---------------------------End Main-------------------------


/* Function name           : systick_handler()                    */
/* Description             : Tick Interrupt handler that deals with state transitions of the FSM and sets pendSV IRQ     */
#[no_mangle]
extern "C" fn systick_handler() {
    unsafe {
        match scheduler_core::curr_task {
             0 => scheduler_core::next_task = 1,
             1 => scheduler_core::next_task = 0,
             _ => { scheduler_core::curr_task = 0;
                    scheduler_core::next_task = 1;
             },
        }
    }
    unsafe {
        //Trigger context switch
        if scheduler_core::curr_task != scheduler_core::next_task {
            SCB.update_icsr(|v| v.with_SCB_ICSR_PENDSVSET_Msk(true)); //Set PendSV to pending
        }
    }
}

/* Function name           : PendSV Interrupt handler                   */
/* Description             : Interrupt handler where the context switch occurs    */
#[inline(never)]
#[no_mangle]
extern "C" fn pendSV_handler () {
    unsafe {
        scheduler_core::context_switch(scheduler_core::next_task, scheduler_core::curr_task,
            scheduler_core::PSP_array.as_mut_ptr() );
    }
}



/* Function name           : trap()                */
/* Description             : Infinite loop to trap execution in case something goes wrong
                            It is compulsary in CSMIS API*/
extern "C" fn trap() { loop {} }


/// The ROM vector table.  This is marked as the program entry point in the
/// linker script, ensuring that any object reachable from this table is
/// preserved in the output binary.
///
/// This is placed in ROM by the linker script because of its assigned
/// `link_section`.  Note that it is not `mut`.
///
/// The `no_mangle` attribute is currently necessary for two reasons:
///
/// 1. To give the table an easily predictable name for use in the linker
///    script.
/// 2. Because `no_mangle` appears to, currently, be the only way to ensure that
///    this symbol is left visible to the linker.
#[no_mangle]
#[link_section=".isr_vector"]
pub static ISR_VECTORS : exc::ExceptionTable = exc::ExceptionTable {
    nmi: Some(trap),
    hard_fault: Some(trap),
    mm_fault: Some(trap),
    bus_fault: Some(trap),
    usage_fault: Some(trap),
    sv_call: Some(trap),
    debug_mon: Some(trap),
    pend_sv: Some(pendSV_handler),
    sys_tick: Some(systick_handler),

    .. exc::empty_exception_table(unsafe { &__STACK_BASE },
                                  arm_m::startup::_reset_vector)
};

/******************************************************************************/

// Application environment.
extern {
    /// This symbol is exported by the linker script, and defines the initial
    /// stack pointer.
    static __STACK_BASE: u32;
}
