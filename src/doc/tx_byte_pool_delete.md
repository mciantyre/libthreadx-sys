Delete memory byte pool

### Description

This service deletes the specified memory byte pool. All threads suspended waiting for memory from this pool are resumed and given a **TX_DELETED** return status.

> **Important:** *It is the application's responsibility to manage the memory area associated with the pool, which is available after this service completes. In addition, the application must prevent use of a deleted pool or memory previously allocated from it.*

### Parameters

- *pool_ptr*: Pointer to a previously created memory pool.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory pool deletion.
- **TX_POOL_ERROR** (0x02) Invalid memory pool pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_create`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]
- [`tx_byte_release`]

