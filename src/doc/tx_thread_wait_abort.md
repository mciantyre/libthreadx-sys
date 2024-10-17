Abort suspension of specified thread

### Description

This service aborts sleep or any other object suspension of the specified thread. If the wait is aborted, a **TX_WAIT_ABORTED** value is returned from the service that the thread was waiting on.

> **Note:** *This service does not release explicit suspension that is made by the tx_thread_suspend service.*

### Parameters

- *thread_ptr*: Pointer to a previously created application thread.

### Return Values

- **TX_SUCCESS** (0x00) Successful thread wait abort.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_WAIT_ABORT_ERROR** (0x1B) Specified thread is not in a waiting state.

### Allowed From

Initialization, threads, timers, and ISRs

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
- [`tx_thread_priority_change`]
- [`tx_thread_relinquish`]
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]

