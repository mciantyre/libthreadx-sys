Retrieve information about mutex

### Description

This service retrieves information from the specified mutex.

### Parameters

- *mutex_ptr*: Pointer to mutex control block.
- *name*: Pointer to destination for the pointer to the mutex's name.
- *count*: Pointer to destination for the ownership count of the mutex.
- *owner*: Pointer to destination for the owning thread's pointer.
- *first_suspended*: Pointer to destination for the pointer to the thread that is first on the suspension list of this mutex.
- *suspended_count*: Pointer to destination for the number of threads currently suspended on this mutex.
- *next_mutex*: Pointer to destination for the pointer of the next created mutex.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex information retrieval.
- **TX_MUTEX_ERROR** (0x1C) Invalid mutex pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_mutex_create`]
- [`tx_mutex_delete`]
- [`tx_mutex_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

