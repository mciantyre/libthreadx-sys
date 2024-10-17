Release fixed-size block of memory

### Description

This service releases a previously allocated block back to its associated memory pool. If there are one or more threads suspended waiting for memory blocks from this pool, the first thread suspended is given this memory block and resumed.

> **Note:** *The application may want to clear the memory block before releasing it to prevent data leaks.*

> **Important:** *The application must prevent using a memory block area after it has been released back to the pool.*

### Parameters

- *block_ptr*: Pointer to the previously allocated memory block.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory block release.
- **TX_PTR_ERROR** (0x03) Invalid pointer to memory block.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_block_allocate`]
- [`tx_block_pool_create`]
- [`tx_block_pool_delete`]
- [`tx_block_pool_info_get`]
- [`tx_block_pool_performance_info_get`]
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_pool_prioritize`]

