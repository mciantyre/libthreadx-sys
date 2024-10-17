Empty messages in message queue

### Description

This service deletes all messages stored in the specified message queue.

If the queue is full, messages of all suspended threads are discarded. Each suspended thread is then resumed with a return status that indicates the message send was successful. If the queue is empty, this service does nothing.

### Parameters

- *queue_ptr*: Pointer to a previously created message queue.

### Return Values

- **TX_SUCCESS** (0x00) Successful message queue flush.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_queue_create`]
- [`tx_queue_delete`]
- [`tx_queue_front_send`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

