Retrieve information about queue

### Description

This service retrieves information about the specified message queue.

### Parameters

- *queue_ptr*: Pointer to a previously created message queue.
- *name*: Pointer to destination for the pointer to the queue's name.
- *enqueued*: Pointer to destination for the number of messages currently in the queue.
- *available_storage*: Pointer to destination for the number of messages the queue currently has space for.
- *first_suspended*: Pointer to destination for the pointer to the thread that is first on the suspension list of this queue.
- *suspended_count*: Pointer to destination for the number of threads currently suspended on this queue.
- *next_queue*: Pointer to destination for the pointer of the next created queue.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful queue information get.
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
- [`tx_queue_performance_info_get`]
- [`tx_queue_performance_system_info_get`]
- [`tx_queue_prioritize`]
- [`tx_queue_receive`]
- [`tx_queue_send`]
- [`tx_queue_send_notify`]

