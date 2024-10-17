Get queue system performance information

### Description

This service retrieves performance information about all the queues in the system.

> **Important:** *The ThreadX library and application must be built with* ***TX_QUEUE_ENABLE_PERFORMANCE_INFO*** *defined for this service to return performance
information.*

### Parameters

- *messages_sent*: Pointer to destination for the total number of send requests performed on all queues.
- *messages_received*: Pointer to destination for the total number of receive requests performed on all queues.
- *empty_suspensions*: Pointer to destination for the total number of queue empty suspensions on all queues.
- *full_suspensions*: Pointer to destination for the total number of queue full suspensions on all queues.
- *full_errors*: Pointer to destination for the total number of queue full errors on all queues.
- *timeouts*: Pointer to destination for the total number of thread suspension timeouts on all queues.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful queue system performance get.
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
- [`tx_queue_performance_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

