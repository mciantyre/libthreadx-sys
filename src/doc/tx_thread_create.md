Create application thread

### Description

This service creates an application thread that starts execution at the specified task entry function. The stack, priority, preemption-threshold, and time-slice are among the attributes specified by the input parameters. In addition, the initial execution state of the thread is also specified.

### Parameters

- *thread_ptr*: Pointer to a thread control block.
- *name_ptr*: Pointer to the name of the thread.
- *entry_function*: Specifies the initial C function for thread execution. When a thread returns from this entry function, it is placed in a *completed* state and suspended indefinitely.
- *entry_input*: A 32-bit value that is passed to the thread's entry function when it first executes. The use for this input is determined exclusively by the application.
- *stack_start*: Starting address of the stack's memory area.
- *stack_size*: Number bytes in the stack memory area. The thread's stack area must be large enough to handle its worst-case function call nesting and local variable usage.
- *priority*: Numerical priority of thread. Legal values range from 0 through (TX_MAX_PRIORITIES-1), where a value of 0 represents the highest priority.
- *preempt_threshold*: Highest priority level (0 through (TX_MAX_PRIORITIES-1)) of disabled preemption. Only priorities higher than this level are allowed to preempt this thread. This value must be less than or equal to the specified priority. A value equal to the thread priority disables preemption-threshold.
- *time_slice*: Number of timer-ticks this thread is allowed to run before other ready threads of the same priority are given a chance to run. Note that using preemption-threshold disables time-slicing. Legal time-slice values range from 1 to 0xFFFFFFFF (inclusive). A value of **TX_NO_TIME_SLICE** (a value of 0) disables time-slicing of this thread.

  > **Note:** *Using time-slicing results in a slight amount of system overhead.   Since time-slicing is only useful in cases where multiple threads   share the same priority, threads having a unique priority should not   be assigned a time-slice.*

- *auto_start*: Specifies whether the thread starts immediately or is placed in a suspended state. Legal options are **TX_AUTO_START** (0x01) and **TX_DONT_START** (0x00). If TX_DONT_START is specified, the application must later call tx_thread_resume in order for the thread to run.

### Return Values

- **TX_SUCCESS** (0x00) Successful thread creation.
- **TX_THREAD_ERROR** (0x0E) Invalid thread control pointer. Either the pointer is NULL or the thread is already created.
- **TX_PTR_ERROR** (0x03) Invalid starting address of the entry point or the stack area is invalid, usually NULL.
- **TX_SIZE_ERROR** (0x05) Size of stack area is invalid. Threads must have at least **TX_MINIMUM_STACK** bytes to execute.
- **TX_PRIORITY_ERROR** (0x0F) Invalid thread priority, which is a value outside the range of (0 through (TX_MAX_PRIORITIES-1)).
- **TX_THRESH_ERROR** (0x18) Invalid preemption threshold specified. This value must be a valid priority less than or equal to the initial priority of the thread.
- **TX_START_ERROR** (0x10) Invalid auto-start selection.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

Yes

### See Also

- [`tx_thread_delete`]
- [`tx_thread_entry_exit_notify`]
- [`tx_thread_identify`]
- [`tx_thread_info_get`]
- [`tx_thread_performance_info_get`]
- [`tx_thread_performance_system_info_get`]
- [`tx_thread_preemption_change`]
- [`tx_thread_priority_change`]
- [`tx_thread_relinquish`]
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

