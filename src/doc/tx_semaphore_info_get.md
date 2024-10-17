Retrieve information about semaphore

### Description

This service retrieves information about the specified semaphore.

### Parameters

- *semaphore_ptr*: Pointer to semaphore control block.
- *name*: Pointer to destination for the pointer to the semaphore's name.
- *current_value*: Pointer to destination for the current semaphore's count.
- *first_suspended*: Pointer to destination for the pointer to the thread that is first on the suspension list of this semaphore.
- *suspended_count*: Pointer to destination for the number of threads currently suspended on this semaphore.
- *next_semaphore*: Pointer to destination for the pointer of the next created semaphore.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) information retrieval.

- **TX_SEMAPHORE_ERROR** (0x0C) Invalid semaphore pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_semaphore_ceiling_put`]
- [`tx_semaphore_create`]
- [`tx_semaphore_delete`]
- [`tx_semaphore_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

