Release bytes back to memory pool

### Description

This service releases a previously allocated memory area back to its associated pool. If there are one or more threads suspended waiting for memory from this pool, each suspended thread is given memory and resumed until the memory is exhausted or until there are no more suspended threads. This process of allocating memory to suspended threads always begins with the first thread suspended.

> **Note:** *The application may want to clear the memory area before releasing it to prevent data leaks.*

> **Important:** *The application must prevent using the memory area after it is released.*

### Parameters

- *memory_ptr*: Pointer to the previously allocated memory area.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory release.
- **TX_PTR_ERROR** (0x03) Invalid memory area pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

Yes

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_create`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]

