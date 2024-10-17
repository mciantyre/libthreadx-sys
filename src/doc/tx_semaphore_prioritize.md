Prioritize semaphore suspension list

### Description

This service places the highest priority thread suspended for an instance of the semaphore at the front of the suspension list. All other threads remain in the same FIFO order they were suspended in.

### Parameters

- *semaphore_ptr*: Pointer to a previously created semaphore.

### Return Values

- **TX_SUCCESS** (0x00) Successful semaphore prioritize.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid counting semaphore pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_semaphore_create`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_get`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_put`]

