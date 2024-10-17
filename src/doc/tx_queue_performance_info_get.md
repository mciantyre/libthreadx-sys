Get queue performance information

### Description

This service retrieves performance information about the specified queue.

> **Important:** *The ThreadX library and application must be built with* ***TX_QUEUE_ENABLE_PERFORMANCE_INFO**: *defined for this service to return performance information.*

### Parameters

- *queue_ptr*: Pointer to previously created queue.
- *messages_sent*: Pointer to destination for the number of send requests performed on this queue.
- *messages_received*: Pointer to destination for the number of receive requests performed on this queue.
- *empty_suspensions*: Pointer to destination for the number of queue empty suspensions on this queue.
- *full_suspensions*: Pointer to destination for the number of queue full suspensions on this queue.
- *full_errors*: Pointer to destination for the number of queue full errors on this queue.
- *timeouts*: Pointer to destination for the number of thread suspension timeouts on this queue.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful queue performance get.
- **TX_PTR_ERROR** (0x03) Invalid queue pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_queue_create`]
- [`tx_queue_delete`]
- [`tx_queue_flush`]
- [`tx_queue_front_send`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

