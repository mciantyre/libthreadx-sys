Retrieve information about event flags group

### Description

This service retrieves information about the specified event flags group.

### Parameters

- *group_ptr*: Pointer to an event flags group control block.
- *name*: Pointer to destination for the pointer to the event flags group's name.
- *current_flags*: Pointer to destination for the current set flags in the event flags group.
- *first_suspended*: Pointer to destination for the pointer to the thread that is first on the suspension list of this event flags group.
- *suspended_count*: Pointer to destination for the number of threads currently suspended on this event flags group.
- *next_group*: Pointer to destination for the pointer of the next created event flags group.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful event group information retrieval.
- **TX_GROUP_ERROR** (0x06) Invalid event group pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_delete`]
- [`tx_event_flags_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]

