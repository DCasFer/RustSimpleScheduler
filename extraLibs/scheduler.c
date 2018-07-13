/*	==================================== OS Data ==============================
This data is used to keep track to the task that is currently running. PSP_array holds all relative
PSPs of each task so that when context switch occurs curr_task=next_task and PSP_array[curr_task] is loaded
into PSP register so next task can be loaded and excute its quantum
*/
unsigned int curr_task = 0;
unsigned int next_task = 0;
unsigned int PSP_array[2] = {0};


/* Function name           : create_task_frame
Param 1: Base address of task_stack arra, which is the stack of each task.
Param 2: Function pointer that points to the function that each task will carry out at run-time
Return: Relative PSP of each task. This will set the system PSP when required
*/
unsigned int create_task_frame(unsigned int* task_stack, unsigned int* core_task) {

	unsigned int PSP = 0;
	unsigned int TASK_STACK_SIZE = 1024;
	unsigned int address_val = 0;
	volatile unsigned long* address_ptr = 0;
	unsigned int xPSR = 0x01000000;

	PSP = ((unsigned int) task_stack) + TASK_STACK_SIZE - (8 * 8);
	address_val = PSP + (14 << 2);
	address_ptr = (volatile unsigned long*) address_val;
	*address_ptr = (unsigned long) core_task;

	address_val = PSP + (15 << 2);
	address_ptr = (volatile unsigned long*) address_val;
	*address_ptr = xPSR;

	return PSP;
}
