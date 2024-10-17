Suspend application thread

### Description

This service suspends the specified application thread. A thread may call this service to suspend itself.

> **Note:** *If the specified thread is already suspended for another reason, this suspension is held internally until the prior suspension is lifted. When that happens, this unconditional suspension of the specified thread is performed. Further unconditional suspension requests have no effect.*

After being suspended, the thread must be resumed by ***tx_thread_resume*** to execute again.

### Parameters

- *thread_ptr*: Pointer to an application thread.

### Return Values

- **TX_SUCCESS** (0x00) Successful thread suspend.
- **TX_THREAD_ERROR** (0x0E) Invalid application thread pointer.
- **TX_SUSPEND_ERROR** (0x14) Specified thread is in a terminated or completed state.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

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
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

