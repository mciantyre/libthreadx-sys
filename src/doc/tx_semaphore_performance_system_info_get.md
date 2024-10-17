Get semaphore system performance information

### Description

This service retrieves performance information about all the semaphores in the system.

> **Important:** *The ThreadX library and application must be built with* ***TX_SEMAPHORE_ENABLE_PERFORMANCE_INFO*** *defined for this service to return performance information*

### Parameters

- *puts*: Pointer to destination for the total number of put requests performed on all semaphores.
- *gets*: Pointer to destination for the total number of get requests performed on all semaphores.
- *suspensions*: Pointer to destination for the total number of thread suspensions on all semaphores.
- *timeouts*: Pointer to destination for the total number of thread suspension timeouts on all semaphores.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) system performance get.
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
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

