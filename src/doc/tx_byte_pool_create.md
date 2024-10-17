Create memory pool of bytes

### Description

This service creates a memory byte pool in the area specified. Initially the pool consists of basically one very large free block. However, the pool is broken into smaller blocks as allocations are made.

### Parameters

- *pool_ptr*: Pointer to a memory pool control block.
- *name_ptr*: Pointer to the name of the memory pool.
- *pool_start*: Starting address of the memory pool. The starting address must be aligned to the size of the ULONG data type.
- *pool_size*: Total number of bytes available for the memory pool.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory pool creation.
- **TX_POOL_ERROR** (0x02) Invalid memory pool pointer. Either the pointer is NULL or the pool is already created.
- **TX_PTR_ERROR** (0x03) Invalid starting address of the pool.
- **TX_SIZE_ERROR** (0x05) Size of pool is invalid.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]
- [`tx_byte_release`]

