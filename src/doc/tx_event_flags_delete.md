Delete event flags group

### Description

This service deletes the specified event flags group. All threads suspended waiting for events from this group are resumed and given a TX_DELETED return status.

> **Important:** *The application must ensure that a set notify callback for this event flags group is completed (or disabled) before deleting the event flags group. In addition, the application must prevent all future use of a deleted event flags group.*

### Parameters

- *group_ptr*: Pointer to a previously created event flags group.

### Return Values

- **TX_SUCCESS** (0x00) Successful event flags group deletion.
- **TX_GROUP_ERROR** (0x06) Invalid event flags group pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_get`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]

