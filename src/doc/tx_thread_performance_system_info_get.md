Get thread system performance information

### Description

This service retrieves performance information about all the threads
in the system.

*The ThreadX library and application must be built with*

***TX_THREAD_ENABLE_PERFORMANCE_INFO*** *defined in order for this service to return performance information.*

### Parameters

- *resumptions*: Pointer to destination for the total number of thread resumptions.
- *suspensions*: Pointer to destination for the total number of thread suspensions.
- *solicited_preemptions*: Pointer to destination for the total number of thread preemptions as a result of a thread calling a ThreadX API service.
- *interrupt_preemptions*: Pointer to destination for the total number of thread preemptions as a result of interrupt processing.
- *priority_inversions*: Pointer to destination for the total number of thread priority inversions.
- *time_slices*: Pointer to destination for the total number of thread time-slices.
- *relinquishes*: Pointer to destination for the total number of thread relinquishes.
- *timeouts*: Pointer to destination for the total number of thread suspension timeouts.
- *wait_aborts*: Pointer to destination for the total number of thread wait aborts.
- *non_idle_returns*: Pointer to destination for the number of times a thread returns to the system when another thread is ready to execute.
- *idle_returns*: Pointer to destination for the number of times a thread returns to the system when no other thread is ready to execute (idle system).

> **Note:** *Supplying a **TX_NULL** for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful thread system performance get.
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
- [`tx_thread_performance_info_get`]
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

