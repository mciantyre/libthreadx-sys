Retrieve information about thread

### Description

This service retrieves information about the specified thread.

### Parameters
- *thread_ptr*: Pointer to thread control block.
- *name*: Pointer to destination for the pointer to the thread's name.
- *state*: Pointer to destination for the thread's current execution state. Possible values are as follows.
    - **TX_READY** (0x00)
    - **TX_COMPLETED** (0x01)
    - **TX_TERMINATED** (0x02)
    - **TX_SUSPENDED** (0x03)
    - **TX_SLEEP** (0x04)
    - **TX_QUEUE_SUSP** (0x05)
    - **TX_SEMAPHORE_SUSP** (0x06)
    - **TX_EVENT_FLAG** (0x07)
    - **TX_BLOCK_MEMORY** (0x08)
    - **TX_BYTE_MEMORY** (0x09)
    - **TX_MUTEX_SUSP** (0x0D)  

- *run_count*: Pointer to destination for the thread's run count.
- *priority*: Pointer to destination for the thread's priority.
- *preemption_threshold*: Pointer to destination for the thread's preemption-threshold.
- *time_slice*: Pointer to destination for the thread's time-slice.
- *next_thread*: Pointer to destination for next created thread pointer.
- *suspended_thread*: Pointer to destination for pointer to next
thread in suspension list.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful thread information retrieval.
- **TX_THREAD_ERROR** (0x0E) Invalid thread control pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_thread_create`]
- [`tx_thread_delete`]
- [`tx_thread_entry_exit_notify`]
- [`tx_thread_identify`]
- [`tx_thread_performance_info_get`]
- [`tx_thread_performance_system_info_get`]
- [`tx_thread_preemption_change`]
- [`tx_thread_priority_change`]
- [`tx_thread_relinquish`]
- [`tx_thread_reset`]
- [`tx_thread_resume`]
- [`tx_thread_sleep`]
- [`tx_thread_stack_error_notify`]
- [`tx_thread_suspend`]
- [`tx_thread_terminate`]
- [`tx_thread_time_slice_change`]
- [`tx_thread_wait_abort`]

