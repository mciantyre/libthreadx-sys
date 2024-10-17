Release ownership of mutex

### Description

This service decrements the ownership count of the specified mutex. If the ownership count is zero, the mutex is made available.

> **Note:** *If priority inheritance was selected during mutex creation, the priority of the releasing thread will be restored to the priority it had when it originally obtained ownership of the mutex. Any other priority changes made to the releasing thread during ownership of the mutex may be undone.*

### Parameters
- mutex_ptr Pointer to the previously created mutex.

### Return Values
- **TX_SUCCESS** (0x00) Successful mutex release.
- **TX_NOT_OWNED** (0x1E) Mutex is not owned by caller.
- **TX_MUTEX_ERROR** (0x1C) Invalid pointer to mutex.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads and timers

### Preemption Possible

Yes

### See Also

- [`tx_mutex_create`]
- [`tx_mutex_delete`]
- [`tx_mutex_get`]
- [`tx_mutex_info_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]

