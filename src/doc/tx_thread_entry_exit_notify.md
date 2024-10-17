Notify application upon thread entry and exit

### Description

This service registers a notification callback function that is called whenever the specified thread is entered or exits. The processing of the notification callback is defined by the application.

> **Note:** The application's thread entry/exit notification callback is not allowed to call any ThreadX API with a suspension option.

### Parameters

- *thread_ptr*: Pointer to previously created thread.
- *entry_exit_notify*: Pointer to application's thread entry/exit notification function. The second parameter to the entry/exit notification function designates if an entry or exit is present. The value **TX_THREAD_ENTRY** (0x00) indicates the thread was entered, while the value **TX_THREAD_EXIT** (0x01) indicates the thread was exited. If this value is **TX_NULL**, notification is disabled.

### Return Values

- **TX_SUCCESS** (0x00) Successful registration of the thread entry/exit notification function.
- **TX_THREAD_ERROR** (0x0E) Invalid thread pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was compiled with notification capabilities disabled.

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

