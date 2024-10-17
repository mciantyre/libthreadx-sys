Get semaphore performance information

### Description

This service retrieves performance information about the specified semaphore.

> **Important:** *The ThreadX library and application must be built with* ***TX_SEMAPHORE_ENABLE_PERFORMANCE_INFO*** *defined for this service to return performance information.*

### Parameters

-  **semaphore_ptr*: Pointer to previously created semaphore.
-  **puts*: Pointer to destination for the number of put requests performed on this semaphore.
-  **gets*: Pointer to destination for the number of get requests performed on this semaphore.
-  **suspensions*: Pointer to destination for the number of thread suspensions on this semaphore.
-  **timeouts*: Pointer to destination for the number of thread suspension timeouts on this semaphore.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful semaphore performance get.
- **TX_PTR_ERROR** (0x03) Invalid semaphore pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

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
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

