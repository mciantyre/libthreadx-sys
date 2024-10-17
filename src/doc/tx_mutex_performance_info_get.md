Get mutex performance information

### Description

This service retrieves performance information about the specified mutex.

> **Important:** *The ThreadX library and application must be built with* ***TX_MUTEX_ENABLE_PERFORMANCE_INFO*** *defined for this service to return performance information.*

### Parameters

- *mutex_ptr*: Pointer to previously created mutex.
- *puts*: Pointer to destination for the number of put requests performed on this mutex.
- *gets*: Pointer to destination for the number of get requests performed on this mutex.
- *suspensions*: Pointer to destination for the number of thread mutex get suspensions on this mutex.
- *timeouts*: Pointer to destination for the number of mutex get suspension timeouts on this mutex.
- *inversions*: Pointer to destination for the number of thread priority inversions on this mutex.
- *inheritances*: Pointer to destination for the number of thread priority inheritance operations on this mutex.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex performance get.
- **TX_PTR_ERROR** (0x03) Invalid mutex pointer.
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
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

