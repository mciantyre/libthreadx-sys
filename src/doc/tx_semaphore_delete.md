Delete counting semaphore

### Description

This service deletes the specified counting semaphore. All threads suspended waiting for a semaphore instance are resumed and given a TX_DELETED return status.

> **Important:** *The application must ensure that a put notify callback for this semaphore is completed (or disabled) before deleting the semaphore. In addition, the application must prevent all future use of a deleted semaphore.*

### Parameters

- *semaphore_ptr*: Pointer to a previously created semaphore.

### Return Values

- **TX_SUCCESS** (0x00) Successful counting semaphore deletion.
- **TX_SEMAPHORE_ERROR** (0x0C) Invalid counting semaphore pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_semaphore_ceiling_put`]
- [`tx_semaphore_create`]
- [`tx_semaphore_get`]
- [`tx_semaphore_info_get`]
- [`tx_semaphore_performance_info_get`]
- [`tx_semaphore_performance_system_info_get`]
- [`tx_semaphore_prioritize`]
- [`tx_semaphore_put`]
- [`tx_semaphore_put_notify`]

