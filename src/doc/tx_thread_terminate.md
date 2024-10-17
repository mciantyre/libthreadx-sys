Terminates application thread

### Description

This service terminates the specified application thread regardless of whether the thread is suspended or not. A thread may call this service to terminate itself.

> **Note:** *It is the application's responsibility to ensure that the thread is in a state suitable for termination. For example, a thread should not be terminated during critical application processing or inside of other middleware components where it could leave such processing in an unknown state.*

> **Important:** *After being terminated, the thread must be reset for it to execute again.*

### Parameters

- *thread_ptr*: Pointer to application thread.

### Return Values
- **TX_SUCCESS** (0x00) Successful thread terminate.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
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
- [`tx_thread_priority_change`]
- [`tx_thread_relinquish`]
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

