Obtain ownership of mutex

### Description

This service attempts to obtain exclusive ownership of the specified mutex. If the calling thread already owns the mutex, an internal counter is incremented and a successful status is returned.

If the mutex is owned by another thread and this thread is higher priority and priority inheritance was specified at mutex create, the lower priority thread's priority will be temporarily raised to that of the calling thread.

> **Note:** *The priority of the lower priority thread owning a mutex with priority inheritance should never be modified by an external thread during mutex ownership.*

### Parameters

- *mutex_ptr*:   <br>Pointer to a previously created mutex.
- *wait_option*: <br>Defines how the service behaves if the mutex is already owned by another thread. The wait options are defined as follows:
  - **TX_NO_WAIT*: (0x00000000) - Selecting TX_NO_WAIT results in an immediate return from this service regardless of whether or not it was successful. *This is the only valid option if the service is called from Initialization.*
  - **TX_WAIT_FOREVER** timeout value (0xFFFFFFFF) - Selecting **TX_WAIT_FOREVER** causes the calling thread to suspend indefinitely until the mutex is available.
  - timeout value (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for the mutex.

### Return Values

- **TX_SUCCESS** (0x00) Successful mutex get operation.
- **TX_DELETED** (0x01) Mutex was deleted while thread was suspended.
- **TX_NOT_AVAILABLE** (0x1D) Service was unable to get ownership of the mutex within the specified time to wait.
- **TX_WAIT_ABORTED** (0x1A) Suspension was aborted by another thread, timer, or ISR.
- **TX_MUTEX_ERROR** (0x1C) Invalid mutex pointer.
- **TX_WAIT_ERROR** (0x04) A wait option other than TX_NO_WAIT was specified on a call from a non-thread.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads and timers

### Preemption Possible

Yes

### See Also

- [`tx_mutex_create`]
- [`tx_mutex_delete`]
- [`tx_mutex_info_get`]
- [`tx_mutex_performance_info_get`]
- [`tx_mutex_performance_system_info_get`]
- [`tx_mutex_prioritize`]
- [`tx_mutex_put`]

