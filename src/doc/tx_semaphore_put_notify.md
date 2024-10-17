Notify application when semaphore is put

### Description

This service registers a notification callback function that is called whenever the specified semaphore is put. The processing of the notification callback is defined by the application.

> **Note:** *The application's semaphore notification callback is not allowed to call any ThreadX API with a suspension option.*

### Parameters

- *semaphore_ptr*: Pointer to previously created semaphore.
- *semaphore_put_notify*: Pointer to application's semaphore put notification function. If this value is TX_NULL, notification is disabled.

### Return Values

- **TX_SUCCESS** (0x00) Successful registration of semaphore put notification.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid semaphore pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was compiled with notification capabilities disabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_semaphore_ceiling_put`]
- [`tx_semaphore_create`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_get`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]

