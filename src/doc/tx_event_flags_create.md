Create event flags group

### Description

This service creates a group of 32 event flags. All 32 event flags in the group are initialized to zero. Each event flag is represented by a single bit.

### Parameters

- *group_ptr*: Pointer to an event flags group control block.
- *name_ptr*: Pointer to the name of the event flags group.

### Return Values

- **TX_SUCCESS** (0x00) Successful event group creation.
- **TX_GROUP_ERROR** (0x06) Invalid event group pointer. Either the pointer is **NULL** or the event group is already created.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_event_flags_delete`]
- [`tx_event_flags_get`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]

