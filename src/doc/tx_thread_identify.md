Retrieves pointer to currently executing thread

### Description

This service returns a pointer to the currently executing thread. If no thread is executing, this service returns a null pointer.

> **Note:** *If this service is called from an ISR, the return value represents the thread running prior to the executing interrupt handler.*

### Parameters

None

### Return Values

- *thread pointer*: Pointer to the currently executing thread. If no thread is executing, the return value is **TX_NULL**.

### Allowed From

Threads and ISRs

### Preemption Possible

No

### See Also

- [`tx_thread_create`]
- [`tx_thread_delete`]
- [`tx_thread_entry_exit_notify`]
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

