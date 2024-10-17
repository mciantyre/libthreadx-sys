Place an instance in counting semaphore

### Description

This service puts an instance into the specified counting semaphore, which in reality increments the counting semaphore by one.

> **Note:** *If this service is called when the semaphore is all ones (OxFFFFFFFF), the new put operation will cause the semaphore to be reset to zero.*

### Parameters

- *semaphore_ptr*: Pointer to the previously created counting semaphore control block.

### Return Values

- **TX_SUCCESS** (0x00) Successful semaphore put.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid pointer to counting semaphore.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_semaphore_ceiling_put`]
- [`tx_semaphore_create`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_get`]
- [`tx_semaphore_put_notify`]

