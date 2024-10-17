Retrieve information about byte pool

### Description

This service retrieves information about the specified memory byte pool.

### Parameters

- *pool_ptr*: Pointer to previously created memory pool.
- *name*: Pointer to destination for the pointer to the byte pool's name.
- *available*: Pointer to destination for the number of available bytes in the pool.
- *fragments*: Pointer to destination for the total number of memory fragments in the byte pool.
- *first_suspended*: Pointer to destination for the pointer to the thread that is first on the suspension list of this byte pool.
- *suspended_count*: Pointer to destination for the number of threads currently suspended on this byte pool.
- *next_pool*: Pointer to destination for the pointer of the next created byte pool.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful pool information retrieve.
- **TX_POOL_ERROR** (0x02) Invalid memory pool pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_create`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]
- [`tx_byte_release`]

