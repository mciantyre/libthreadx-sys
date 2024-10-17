Delete mutual exclusion mutex

### Description

This service deletes the specified mutex. All threads suspended waiting for the mutex are resumed and given a **TX_DELETED** return status.

> **Note:** *It is the application's responsibility to prevent use of a deleted mutex.*

### Parameters

- *mutex_ptr*: Pointer to a previously created mutex.

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex deletion.
- **TX_MUTEX_ERROR** (0x1C) Invalid mutex pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_mutex_create`]
- [`tx_mutex_get`]
- [`tx_mutex_info_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

