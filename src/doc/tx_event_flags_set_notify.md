Notify application when event flags are set

### Description

This service registers a notification callback function that is called whenever one or more event flags are set in the specified event flags group. The processing of the notification callback is defined by the

### Parameters
- *group_ptr*: Pointer to previously created event flags group.
- *events_set_notify*: Pointer to application's event flags set notification function. If this value is TX_NULL, notification is disabled.

### Return Values

- **TX_SUCCESS** (0x00) Successful registration of event flags set notification.
- **TX_GROUP_ERROR** (0x06) Invalid event flags group pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was compiled with notification capabilities disabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_delete`]
- [`tx_event_flags_get`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]

