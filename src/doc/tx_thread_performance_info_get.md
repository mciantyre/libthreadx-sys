Get thread performance information

### Description

This service retrieves performance information about the specified thread.

> **Important:** *The ThreadX library and application must be built with* ***TX_THREAD_ENABLE_PERFORMANCE_INFO*** *defined in order for this service to return performance information.*

### Parameters
- *thread_ptr*: Pointer to previously created thread.
- *resumptions*: Pointer to destination for the number of resumptions of this thread.
- *suspensions*: Pointer to destination for the number of suspensions of this thread.
- *solicited_preemptions*: Pointer to destination for the number of preemptions as a result of a ThreadX API service call made by this thread.
- *interrupt_preemptions*: Pointer to destination for the number of preemptions of this thread as a result of interrupt processing.
- *priority_inversions*: Pointer to destination for the number of priority inversions of this thread.
- *time_slices*: Pointer to destination for the number of time-slices of this thread.
- *relinquishes*: Pointer to destination for the number of thread relinquishes performed by this thread.
- *timeouts*: Pointer to destination for the number of suspension timeouts on this thread.
- *wait_aborts*: Pointer to destination for the number of wait aborts performed on this thread.
- *last_preempted_by*: Pointer to destination for the thread pointer that last preempted this thread.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful thread performance get.
- **TX_PTR_ERROR** (0x03) Invalid thread pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_thread_create`]
- [`tx_thread_delete`]
- [`tx_thread_entry_exit_notify`]
- [`tx_thread_identify`]
- [`tx_thread_info_get`]
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

