Reset thread

### Description

This service resets the specified thread to execute at the entry point defined at thread creation. The thread must be in either a **TX_COMPLETED** or **TX_TERMINATED** state for it to be reset

> **Important:** *The thread must be resumed for it to execute again.*

### Parameters

- *thread_ptr*: Pointer to a previously created thread.

### Return Values

- **TX_SUCCESS** (0x00) Successful thread reset.
- **TX_NOT_DONE** (0x20) Specified thread is not in a **TX_COMPLETED** or **TX_TERMINATED** state.
- **TX_THREAD_ERROR** (0x0E) Invalid thread pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

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
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

