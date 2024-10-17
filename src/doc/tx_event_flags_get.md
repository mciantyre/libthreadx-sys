Get event flags from event flags group

### Description

This service retrieves event flags from the specified event flags group. Each event flags group contains 32 event flags. Each flag is represented by a single bit. This service can retrieve a variety of event flag combinations, as selected by the input parameters.

### Parameters

- *group_ptr*: <br>Pointer to a previously created event flags group.
- *requested_flags*: <br>32-bit unsigned variable that represents the requested event flags.
- *get_option*: <br>Specifies whether all or any of the requested event flags are required. The following are valid selections:

  - **TX_AND** (0x02)
  - **TX_AND_CLEAR** (0x03)
  - **TX_OR** (0x00)
  - **TX_OR_CLEAR** (0x01)

    Selecting TX_AND or TX_AND_CLEAR specifies that all event flags must be present in the group. Selecting TX_OR or TX_OR_CLEAR     specifies that any event flag is satisfactory. Event flags that satisfy the request are cleared (set to zero) if TX_AND_CLEAR or TX_OR_CLEAR are specified.

- *actual_flags_ptr*: <br>Pointer to destination of where the retrieved event flags are placed. Note that the actual flags obtained may contain flags that were not requested.
- *wait_option*:  <br>Defines how the service behaves if the selected event flags are not set. The wait options are defined as follows:
  - **TX_NO_WAIT** (0x00000000) - Selecting TX_NO_WAIT results in an immediate return from this service regardless of whether or not it was successful. This is the only valid option if the service is called from a non-thread; e.g., Initialization, timer, or ISR.
  - **TX_WAIT_FOREVER** timeout value  (0xFFFFFFFF) - Selecting TX_WAIT_FOREVER causes the calling thread to suspend indefinitely until the event flags are available.
  - timeout value (0x00000001 through 0xFFFFFFFE) - Selecting a numeric value (1-0xFFFFFFFE) specifies the maximum number of timer-ticks to stay suspended while waiting for the event flags.

### Return Values

- **TX_SUCCESS** (0x00) Successful event flags get.
- **TX_DELETED** (0x01) Event flags group was deleted while thread was suspended.
- **TX_NO_EVENTS** (0x07) Service was unable to get the specified events within the specified time to wait.
- **TX_WAIT_ABORTED** (0x1A) Suspension was aborted by another thread, timer, or ISR.
- **TX_GROUP_ERROR** (0x06) Invalid event flags group pointer.
- **TX_PTR_ERROR** (0x03) Invalid pointer for actual event flags.
- **TX_WAIT_ERROR** (0x04) A wait option other than TX_NO_WAIT was specified on a call from a nonthread.
- **TX_OPTION_ERROR** (0x08) Invalid get-option was specified.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_delete`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]

