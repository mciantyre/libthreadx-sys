Get mutex system performance information

### Description

This service retrieves performance information about all the mutexes in the system.

> **Important:** *The ThreadX library and application must be built with* **TX_MUTEX_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *puts*: Pointer to destination for the total number of put requests performed on all mutexes.
- *gets*: Pointer to destination for the total number of get requests performed on all mutexes.
- *suspensions*: Pointer to destination for the total number of thread mutex get suspensions on all mutexes.
- *timeouts*: Pointer to destination for the total number of mutex get suspension timeouts on all mutexes.
- *inversions*: Pointer to destination for the total number of thread priority inversions on all mutexes.
- *inheritances*: Pointer to destination for the total number of thread priority inheritance operations on all mutexes.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex system performance get.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_mutex_create`]
- [`tx_mutex_delete`]
- [`tx_mutex_get`]
- [`tx_mutex_info_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

