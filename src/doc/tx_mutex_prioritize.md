Prioritize mutex suspension list

### Description

This service places the highest priority thread suspended for ownership of the mutex at the front of the suspension list. All other threads remain in the same FIFO order they were suspended in.

### Parameters

- *mutex_ptr*: Pointer to the previously created mutex.

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex prioritize.
- **TX_MUTEX_ERROR** (0x1C) Invalid mutex pointer.

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
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_put`]

