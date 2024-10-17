Allocate fixed-size block of memory

### Description

This service allocates a fixed-size memory block from the specified memory pool. The actual size of the memory block is determined during memory pool creation.

> **Important:** *It is important to ensure application code does not write outside the allocated memory block. If this happens, corruption occurs in an adjacent (usually subsequent) memory block. The results are unpredictable and often fatal!*

### Parameters

- *pool_ptr*: <br>Pointer to a previously created memory block pool.
- *block_ptr*: <br>Pointer to a destination block pointer. On successful allocation, the address of the allocated memory block is placed where this parameter points.
- *wait_option*: <br>Defines how the service behaves if there are no memory blocks available. The wait options are defined as follows:
  - **TX_NO_WAIT** (0x00000000) - Selecting **TX_NO_WAIT** results in an immediate return from this service regardless if it was successful or not. *This is the only valid option if the service is called from a non-thread; e.g., Initialization, timer, or ISR*.
  - **TX_WAIT_FOREVER** (0xFFFFFFF) - Selecting **TX_WAIT_FOREVER** causes the calling thread to suspend indefinitely until a memory block is available.
  - *timeout value* (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for a memory block.

### Return Values

- **TX_SUCCESS**	(0x00)	Successful memory block allocation.
- **TX_DELETED**	(0x01)	Memory block pool was deleted while thread was suspended.
- **TX_NO_MEMORY**	(0x10)	Service was unable to allocate a block of memory within the specified time to wait.
- **TX_WAIT_ABORTED**	(0x1A)	Suspension was aborted by another thread, timer or ISR.
- **TX_POOL_ERROR**	(0x02)	Invalid memory block pool pointer.
- **TX_WAIT_ERROR**	(0x04)	A wait option other than TX_NO_WAIT was specified on a call from a nonthread.
- **TX_PTR_ERROR**	(0x03)	Invalid pointer to destination pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_block_pool_create`]
- [`tx_block_pool_delete`]
- [`tx_block_pool_info_get`]
- [`tx_block_pool_performance_info_get`]
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_pool_prioritize`]
- [`tx_block_release`]

