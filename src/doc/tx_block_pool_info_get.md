Retrieve information about block pool

### Description

This service retrieves information about the specified block memory pool.

### Parameters

- *pool_ptr**	Pointer to previously created memory block pool.
- *name**	Pointer to destination for the pointer to the block pool's name.
- *available**	Pointer to destination for the number of available blocks in the block pool.
- *total_blocks**	Pointer to destination for the total number of blocks in the block pool.
- *first_suspended**	Pointer to destination for the pointer to the thread that is first on the suspension list of this block pool.
- *suspended_count**	Pointer to destination for the number of threads currently suspended on this block pool.
- *next_pool**	Pointer to destination for the pointer of the next created block pool.

> **Note:** *Supplying a TX_NULL for any parameter indicates the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful block pool information retrieve.
- **TX_POOL_ERROR** (0x02) Invalid memory block pool pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_block_allocate`]
- [`tx_block_pool_create`]
- [`tx_block_pool_delete`]
- [`tx_block_pool_info_get`]
- [`tx_block_pool_performance_info_get`]
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_pool_prioritize`]
- [`tx_block_release`]

