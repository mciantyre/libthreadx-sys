Prioritize byte pool suspension list

### Description

This service places the highest priority thread suspended for memory on this pool at the front of the suspension list. All other threads remain in the same FIFO order they were suspended in.

### Parameters

- *pool_ptr*: Pointer to a memory pool control block.

### Return Values

- **TX_SUCCESS** (0x00) Successful memory pool prioritize.
- **TX_POOL_ERROR** (0x02) Invalid memory pool pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_create`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_release`]

