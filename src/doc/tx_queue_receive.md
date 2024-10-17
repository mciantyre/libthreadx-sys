Get message from message queue

### Description

This service retrieves a message from the specified message queue. The retrieved message is **copied** from the queue into the memory area specified by the destination pointer. That message is then removed from the queue.

> **Important:** *The specified destination memory area must be large enough to hold the message; i.e., the message destination pointed to by* ***destination_ptr*** *must be at least as large as the message size for this queue. Otherwise, if the destination is not large enough, memory corruption occurs in the following memory area.*

### Parameters

- *queue_ptr*: <br>Pointer to a previously created message queue.
- *destination_ptr*: <br>Location of where to copy the message.
- *wait_option*: <br>Defines how the service behaves if the message queue is empty. The wait options are defined as follows:
  - **TX_NO_WAIT*: (0x00000000) - Selecting TX_NO_WAIT results in an immediate return from this service regardless of whether or not it was successful. This is the only valid option if the service is called from a non-thread; e.g.,  Initialization, timer, or ISR.
  - **TX_WAIT_FOREVER** (0xFFFFFFFF) - Selecting TX_WAIT_FOREVER causes the calling thread to suspend indefinitely until a message is available.
  - timeout value (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for a message.

### Return Values

- **TX_SUCCESS** (0x00) Successful retrieval of message.
- **TX_DELETED** (0x01) Message queue was deleted while thread was suspended.
- **TX_QUEUE_EMPTY** (0x0A) Service was unable to retrieve a message because the queue was empty for the duration of the specified time to wait.
- **TX_WAIT_ABORTED** (0x1A) Suspension was aborted by another thread, timer, or ISR.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer.
- **TX_PTR_ERROR** (0x03) Invalid destination pointer for message.
- **TX_WAIT_ERROR** (0x04) A wait option other than TX_NO_WAIT was specified on a call from a nonthread.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_queue_create`]
- [`tx_queue_delete`]
- [`tx_queue_flush`]
- [`tx_queue_front_send`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

