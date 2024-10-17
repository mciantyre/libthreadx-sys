Resume suspended application thread

### Description

This service resumes or prepares for execution a thread that was previously suspended by a ***tx_thread_suspend*** call. In addition, this service resumes threads that were created without an automatic start.

### Parameters

- *thread_ptr*: Pointer to a suspended application thread.

### Return Values

- **TX_SUCCESS** (0x00) Successful thread resume.
- **TX_SUSPEND_LIFTED** (0x19) Previously set delayed suspension was lifted.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_RESUME_ERROR** (0x12) Specified thread is not suspended or was previously suspended by a service other than ***tx_thread_suspend***.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

TX_THREAD my_thread;

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
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

