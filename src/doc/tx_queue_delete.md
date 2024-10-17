Delete message queue

### Description

This service deletes the specified message queue. All threads suspended waiting for a message from this queue are resumed and given a TX_DELETED return status.

> **Important:** *The application must ensure that any send notify callback for this queue is completed (or disabled) before deleting the queue. In addition, the application must prevent any future use of a deleted queue.* <br><br>*It is also the application's responsibility to manage the memory area associated with the queue, which is available after this service completes.*

### Parameters

- *queue_ptr*: Pointer to a previously created message queue.

### Return Values

- **TX_SUCCESS** (0x00) Successful message queue deletion.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_queue_create`]
- [`tx_queue_flush`]
- [`tx_queue_front_send`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

