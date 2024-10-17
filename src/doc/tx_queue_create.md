Create message queue

### Description

This service creates a message queue that is typically used for interthread communication. The total number of messages is calculated from the specified message size and the total number of bytes in the queue.

> **Note:** *If the total number of bytes specified in the queue's memory area is not evenly divisible by the specified message size, the remaining bytes in the memory area are not used.*

### Parameters

- *queue_ptr*: Pointer to a message queue control block.
- *name_ptr*: Pointer to the name of the message queue.
- *message_size*: Specifies the size of each message in the queue. Message sizes range from 1 32-bit word to 16 32-bit words. Valid message size options are numerical values from 1 through 16, inclusive.
- *queue_start*: Starting address of the message queue. The starting address must be aligned to the size of the ULONG data type.
- *queue_size*: Total number of bytes available for the message queue.

### Return Values

- **TX_SUCCESS** (0x00) Successful message queue creation.
- **TX_QUEUE_ERROR** (0x09) Invalid message queue pointer. Either the pointer is NULL or the queue is already created.
- **TX_PTR_ERROR** (0x03) Invalid starting address of the message queue.
- **TX_SIZE_ERROR** (0x05) Size of message queue is invalid.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_queue_delete`]
- [`tx_queue_flush`]
- [`tx_queue_front_send`]
- [`tx_queue_info_get`]
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

