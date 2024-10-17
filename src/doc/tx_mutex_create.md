Create mutual exclusion mutex

### Description

This service creates a mutex for inter-thread mutual exclusion for resource protection.

### Parameters

- *mutex_ptr*: Pointer to a mutex control block.
- *name_ptr*: Pointer to the name of the mutex.
- *priority_inherit*: Specifies whether or not this mutex supports priority inheritance. If this value is TX_INHERIT, then priority inheritance is supported. However, if TX_NO_INHERIT is specified, priority inheritance is not supported by this mutex.

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex creation.
- **TX_MUTEX_ERROR** (0x1C) Invalid mutex pointer. Either the pointer is NULL or the mutex is already created.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.
- **TX_INHERIT_ERROR** (0x1F) Invalid priority inherit parameter.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_mutex_delete`]
- [`tx_mutex_get`]
- [`tx_mutex_info_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

