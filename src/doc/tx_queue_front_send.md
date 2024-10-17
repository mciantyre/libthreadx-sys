Send message to the front of queue

### Description

This service sends a message to the front location of the specified message queue. The message is **copied** to the front of the queue from the memory area specified by the source pointer.

### Parameters

- *queue_ptr*: <br>Pointer to a message queue control block.
- *source_ptr*: <br>Pointer to the message.
- *wait_option*:  <br>Defines how the service behaves if the message queue is full. The wait options are defined as follows:
  - **TX_NO_WAIT*: (0x00000000) - Selecting TX_NO_WAIT results in an immediate return from this service regardless of whether or not it was successful. *This is the only valid option if the service is called from a non-thread; e.g., Initialization, timer, or ISR.*
  - **TX_WAIT_FOREVER** (0xFFFFFFFF) - Selecting TX_WAIT_FOREVER causes the calling thread to suspend indefinitely until there is room in the queue.
  - timeout value (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for room in the queue.

### Return Values

- **TX_SUCCESS** (0x00) Successful sending of message.
- **TX_DELETED** (0x01) Message queue was deleted while thread was suspended.
- **TX_QUEUE_FULL** (0x0B) Service was unable to send message because the queue was full for the duration of the specified time to wait.
- **TX_WAIT_ABORTED** (0x1A) Suspension was aborted by another thread, timer, or ISR.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer.
- **TX_PTR_ERROR** (0x03) Invalid source pointer for message.
- **TX_WAIT_ERROR** (0x04) A wait option other than TX_NO_WAIT was specified on a call from a non-thread.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_queue_create`]
- [`tx_queue_delete`]
- [`tx_queue_flush`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

