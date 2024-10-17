Allocate bytes of memory

### Description

This service allocates the specified number of bytes from the specified memory byte pool.

> **Important:** *It is important to ensure application code does not write outside the allocated memory block. If this happens, corruption occurs in an adjacent (usually subsequent) memory block. The results are unpredictable and often fatal!*

> **Note:** *The performance of this service is a function of the block size and the amount of fragmentation in the pool. Hence, this service should not be used during time-critical threads of execution.*

### Parameters

- *pool_ptr*: <br>Pointer to a previously created memory block pool.
- *memory_ptr*: <br>Pointer to a destination memory pointer. On successful allocation, the address of the allocated memory area is placed where this parameter points to.
- *memory_size*: <br>Number of bytes requested.
- *wait_option*: <br>Defines how the service behaves if there is not enough memory available. The wait options are defined as follows:
  - **TX_NO_WAIT** (0x00000000) - Selecting **TX_NO_WAIT** results in an immediate return from this service regardless of whether or not it was successful. *This is the only valid option if the service is called from initialization.*
  - **TX_WAIT_FOREVER** 0xFFFFFFFF) - Selecting **TX_WAIT_FOREVER** causes the calling thread to suspend indefinitely until enough memory is available.
  - *timeout value* (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for the memory.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory allocation.
- **TX_DELETED** (0x01) Memory pool was deleted while thread was suspended.
- **TX_NO_MEMORY** (0x10) Service was unable to allocate the memory within the specified time to wait.
- **TX_WAIT_ABORTED** (0x1A) Suspension was aborted by another thread, timer, or ISR.
- **TX_POOL_ERROR** (0x02) Invalid memory pool pointer.
- **TX_PTR_ERROR** (0x03) Invalid pointer to destination pointer.
- **TX_SIZE_ERROR** (0X05) Requested size is zero or larger than the pool.
- **TX_WAIT_ERROR** (0x04) A wait option other than TX_NO_WAIT was specified on a call from a nonthread.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

Yes

### See Also

- [`tx_byte_pool_create`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]
- [`tx_byte_release`]

