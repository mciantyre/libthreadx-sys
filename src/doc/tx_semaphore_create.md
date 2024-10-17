Create counting semaphore

### Description

This service creates a counting semaphore for inter-thread synchronization. The initial semaphore count is specified as an input parameter.

### Parameters

- *semaphore_ptr*: Pointer to a semaphore control block.
- *name_ptr*: Pointer to the name of the semaphore.
- *initial_count*: Specifies the initial count for this semaphore. Legal values range from 0x00000000 through 0xFFFFFFFF.

### Return Values

- **TX_SUCCESS** (0x00) Successful semaphore creation.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid semaphore pointer. Either the pointer is NULL or the semaphore is already created.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_semaphore_ceiling_put`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_get`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

