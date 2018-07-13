pub const NUM_TASK : usize = 2;
pub const TASK_STACK_SIZE : usize = 1024;


pub const priority: u8 = 0xFF;           //Lowest possible priority
pub const PendSV_IRQ: u32 = 0x00000038; //Cortex M4 PendSV vector adddress 0x00000038 or exception number is 14


/* Function name           : set_control()                 */
/* Description             : Set control register to 0x03, which switches to unpriveledge mode;*/
pub fn set_control () {
    unsafe {
        asm!("MSR control, $0"
             :: "r"(0x3)
             : "memory")
    }
}

/* Function name           : set_PSP()
Param 1: Array with task PSPs
Param 2: Index of PSP_array
*/
/* Description             : Sets PSP register to the first task.
                            The same as __set_PSP((PSP_array[curr_task] + 8 * 8));
                            8 * 8 because double stack alignment*/
pub unsafe fn set_PSP (PSP_array_: &mut [u32], curr_task_:& u32) {
    let _curr_task = *curr_task_ as usize;
        asm!("MSR psp, $0\n"
             :: "r"(PSP_array_[_curr_task] + 8 * 8)
             : "sp")
}

/* Description             : Static C library from c_code/libscheduler.a.    */
#[link(name = "scheduler")]
extern {
    pub fn create_task_frame (task_stack: *mut u32, core_task : fn() ) -> u32;
    pub static mut curr_task: u32;
    pub static mut next_task: u32;
    pub static mut PSP_array: [u32; NUM_TASK];
}

/* Description: Static assembly library from c_code/libmemset */
#[link(name = "memset")]
extern {
    pub fn __aeabi_memclr (r0: *mut usize, r1 : usize);   //void __aeabi_memclr(void *r0, size_t r1)
    pub fn __aeabi_memclr4 (r0: *mut usize , r1 : usize ); //void __aeabi_memclr4(void *r0, size_t r1)
    pub fn __aeabi_memclr8 (r0: *mut usize, r1 : *mut u32 ); //void __aeabi_memclr8(void *r0, size_t r1)
}

/*Description: Static assembly library from c_code/libcontextSwitch */
#[link(name = "contextSwitch")]
extern {
    pub fn context_switch(next_task: u32, curr_task: u32, PSP_array:  *mut u32);
}

/* Function name           : scheduler_task                    */
/* Description             : Defines each task of the scheduler    */
pub enum scheduler_task {
    task_1 = 1,
    task_2 = 2,
    task_3 = 3,
}
