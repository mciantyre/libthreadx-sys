Create pool of fixed-size memory blocks

### Description

This service creates a pool of fixed-size memory blocks. The memory area specified is divided into as many fixed-size memory blocks as possible using the formula:

**total blocks** = (**total bytes**) /
(**block size** + sizeof(void *))

> **Note:** *Each memory block contains one pointer of overhead that is invisible to the user and is represented by the "sizeof(void *)" in the preceding formula.*

### Parameters

- *pool_ptr**	Pointer to a memory block pool control block.
- *name_ptr**	Pointer to the name of the memory block pool.
- *block_size**	Number of bytes in each memory block.
- *pool_start**	Starting address of the memory block pool. The starting address must be aligned to the size of the ULONG data type.
- *pool_size**	Total number of bytes available for the memory block pool.

### Return Values

- **TX_SUCCESS**	(0x00)	Successful memory block pool creation.
- **TX_POOL_ERROR**	(0x02)	Invalid memory block pool pointer. Either the pointer is NULL or the pool is already created.
- **TX_PTR_ERROR**	(0x03)	Invalid starting address of the pool.
- **TX_CALLER_ERROR**	(0x13)	Invalid caller of this service.
- **TX_SIZE_ERROR**	(0x05)	Size of pool is invalid.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_block_allocate`]
- [`tx_block_pool_delete`]
- [`tx_block_pool_info_get`]
- [`tx_block_pool_performance_info_get`]
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_pool_prioritize`]
- [`tx_block_release`]

