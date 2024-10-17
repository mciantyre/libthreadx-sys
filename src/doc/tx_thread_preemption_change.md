Change preemption-threshold of application thread

### Description

This service changes the preemption-threshold of the specified thread. The preemption-threshold prevents preemption of the specified thread by threads equal to or less than the preemption-threshold value.

> **Note:** *Using preemption-threshold disables time-slicing for the specified thread.*

### Parameters
- *thread_ptr*: Pointer to a previously created application thread.
- *new_threshold*: New preemption-threshold priority level (0 through (TX_MAX_PRIORITIES-1)).
- *old_threshold*: Pointer to a location to return the previous preemption-threshold.

### Return Values

- **TX_SUCCESS** (0x00) Successful preemption-threshold change.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_THRESH_ERROR** (0x18) Specified new preemption-threshold is not a valid thread priority (a value other than (0 through (**TX_MAX_PRIORITIES**-1)) or is greater than (lower priority) than the current thread priority.
- **TX_PTR_ERROR** (0x03) Invalid pointer to previous preemption threshold storage location.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads and timers

### Preemption Possible

Yes

### See Also

- [`tx_thread_create`]
- [`tx_thread_delete`]
- [`tx_thread_entry_exit_notify`]
- [`tx_thread_identify`]
- [`tx_thread_info_get`]
- [`tx_thread_performance_info_get`]
- [`tx_thread_performance_system_info_get`]
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

