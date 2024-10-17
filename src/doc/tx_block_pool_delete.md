Delete memory block pool

### Description

This service deletes the specified block-memory pool. All threads suspended waiting for a memory block from this pool are resumed and given a **TX_DELETED** return status.

> **Note:** *It is the application's responsibility to manage the memory area associated with the pool, which is available after this service completes. In addition, the application must prevent use of a deleted pool or its former memory blocks.*

### Parameters

- *pool_ptr*: Pointer to a previously created memory block pool.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory block pool deletion.
- **TX_POOL_ERROR** (0x02) Invalid memory block pool pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_block_allocate`]
- [`tx_block_pool_create`]
- [`tx_block_pool_info_get`]
- [`tx_block_pool_performance_info_get`]
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_pool_prioritize`]
- [`tx_block_release`]

