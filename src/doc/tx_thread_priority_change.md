Change priority of application thread

### Description

This service changes the priority of the specified thread. Valid priorities range from 0 through (TX_MAX_PRIORITIES-1), where 0 represents the highest priority level.

> **Important:** *The preemption-threshold of the specified thread is automatically set to the new priority. If a new threshold is desired, the ***tx_thread_preemption_change*** service must be used after this call.*

### Parameters

- *thread_ptr*: Pointer to a previously created application thread.
- *new_priority*: New thread priority level (0 through (TX_MAX_PRIORITIES-1)).
- *old_priority*: Pointer to a location to return the thread's previous priority.

### Return Values

- **TX_SUCCESS** (0x00) Successful priority change.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_PRIORITY_ERROR** (0x0F) Specified new priority is not valid (a value other than (0 through (TX_MAX_PRIORITIES-1)).
- **TX_PTR_ERROR** (0x03) Invalid pointer to previous priority storage location.
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
- [`tx_thread_preemption_change`]
- [`tx_thread_relinquish`]
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

