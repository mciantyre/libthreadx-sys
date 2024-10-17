Prioritize block pool suspension list

### Description

This service places the highest priority thread suspended for a block of memory on this pool at the front of the suspension list. All other threads remain in the same FIFO order they were suspended in.

### Parameters

- *pool_ptr*: Pointer to a memory block pool control block.

### Return Values

- **TX_SUCCESS** (0x00) Successful block pool prioritize.
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
- [`tx_block_release`]

