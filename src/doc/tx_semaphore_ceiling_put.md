Place an instance in counting semaphore with ceiling

### Description

This service puts an instance into the specified counting semaphore, which in reality increments the counting semaphore by one. If the counting semaphore's current value is greater than or equal to the specified ceiling, the instance will not be put and a TX_CEILING_EXCEEDED error will be returned.

### Parameters

- *semaphore_ptr*: Pointer to previously created semaphore.
- *ceiling*: Maximum limit allowed for the semaphore (valid values range from 1 through 0xFFFFFFFF).

### Return Values

- **TX_SUCCESS (0x00)** Successful semaphore ceiling put.
- **TX_CEILING_EXCEEDED** (0x21) Put request exceeds ceiling.
- **TX_INVALID_CEILING** (0x22) An invalid value of zero was supplied for ceiling.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid semaphore pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_semaphore_create`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_get`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

