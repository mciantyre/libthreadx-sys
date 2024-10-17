Prioritize queue suspension list

### Description

This service places the highest priority thread suspended for a message (or to place a message) on this queue at the front of the suspension list.

All other threads remain in the same FIFO order they were suspended in.

### Parameters

- *queue_ptr*: Pointer to a previously created message queue.

### Return Values

- **TX_SUCCESS** (0x00) Successful queue prioritize.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer.

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
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

