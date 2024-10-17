Get byte pool performance information

### Description

This service retrieves performance information about the specified memory byte pool.

> **Important:** *The ThreadX library and application must be built with* **TX_BYTE_POOL_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *pool_ptr*: Pointer to previously created memory byte pool.
- *allocates*: Pointer to destination for the number of allocate requests performed on this pool.
- *releases*: Pointer to destination for the number of release requests performed on this pool.
- *fragments_searched*: Pointer to destination for the number of internal memory fragments searched during allocation requests on this pool.
- *merges*: Pointer to destination for the number of internal memory blocks merged during allocation requests on this pool.
- *splits*: Pointer to destination for the number of internal memory blocks split (fragments) created during allocation requests on this pool.
- *suspensions*: Pointer to destination for the number of thread allocation suspensions on this pool.
- *timeouts*: Pointer to destination for the number of allocate suspension timeouts on this pool.

> **Note:** *Supplying a TX_NULL for any parameter indicates the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful byte pool performance get.
- **TX_PTR_ERROR** (0x03) Invalid byte pool pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_byte_allocate`]
- [`tx_byte_pool_create`]
- [`tx_byte_pool_delete`]
- [`tx_byte_pool_info_get`]
- [`tx_byte_pool_performance_system_info_get`]
- [`tx_byte_pool_prioritize`]
- [`tx_byte_release`]

