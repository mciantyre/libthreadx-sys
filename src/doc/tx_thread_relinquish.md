Relinquish control to other application threads

### Description

This service relinquishes processor control to other ready-to-run threads at the same or higher priority.

> **Note:** *In addition to relinquishing control to threads of the same priority, this service also relinquishes control to the highest-priority thread prevented from execution because of the current thread's preemption-threshold setting.*

### Parameters

None

### Return Values

None

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
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

