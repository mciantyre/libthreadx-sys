Changes time-slice of application thread

### Description

This service changes the time-slice of the specified application thread. Selecting a time-slice for a thread insures that it won't execute more than the specified number of timer ticks before other threads of the same or higher priorities have a chance to execute.

> **Note:** *Using preemption-threshold disables time-slicing for the specified thread.*

### Parameters

- *thread_ptr*: Pointer to application thread.
- *new_time_slice*: New time slice value. Legal values include TX_NO_TIME_SLICE and numeric values from 1 through 0xFFFFFFFF.
- *old_time_slice*: Pointer to location for storing the previous timeslice value of the specified thread.

### Return Values

- **TX_SUCCESS** (0x00) Successful time-slice chance.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_PTR_ERROR** (0x03) Invalid pointer to previous time-slice storage location.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads and timers

### Preemption Possible

No

### See Also

- [`tx_thread_create`]
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
- [`tx_thread_wait_abort`]

