Get block pool system performance information

### Description

This service retrieves performance information about all memory block pools in the application.

> **Important:** *The ThreadX library and application must be built with* **TX_BLOCK_POOL_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *allocates*: Pointer to destination for the total number of allocate requests performed on all block pools.
- *releases*: Pointer to destination for the total number of release requests performed on all block pools.
- *suspensions*: Pointer to destination for the total number of thread allocation suspensions on all block pools.
- *timeouts*: Pointer to destination for the total number of allocate suspension timeouts on all block pools.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful block pool system performance get.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

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
- [`tx_block_pool_prioritize`]
- [`tx_block_release`]

