Get block pool performance information

### Description

This service retrieves performance information about the specified memory block pool.

> **Important:** *The ThreadX library and application must be built with* **TX_BLOCK_POOL_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *pool_ptr**	Pointer to previously created memory block pool.
- *allocates**	Pointer to destination for the number of allocate requests performed on this pool.
- *releases**	Pointer to destination for the number of release requests performed on this pool.
- *suspensions**	Pointer to destination for the number of thread allocation suspensions on this pool.
- *timeouts**	Pointer to destination for the number of allocate suspension timeouts on this pool.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS**	(0x00)	Successful block pool performance get.
- **TX_PTR_ERROR**	(0x03)	Invalid block pool pointer.
- **TX_FEATURE_NOT_ENABLED**	(0xFF)	The system was not compiled with performance information enabled.

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
- [`tx_block_pool_performance_system_info_get`]
- [`tx_block_release`]

