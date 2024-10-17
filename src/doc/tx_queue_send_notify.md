Notify application when message is sent to queue

### Description

This service registers a notification callback function that is called whenever a message is sent to the specified queue. The processing of the notification callback is defined by the application.

> **Note:** *The application's queue send notification callback is not allowed to call any ThreadX API with a suspension option.*

### Parameters

- *queue_ptr*: Pointer to previously created queue.
- *queue_send_notify*: Pointer to application's queue send notification function. If this value is TX_NULL, notification is disabled.

### Return Values

- **TX_SUCCESS** (0x00) Successful registration of queue send notification.
- **TX_QUEUE_ERROR** (0x09) Invalid queue pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was compiled with notification capabilities disabled.

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
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]

