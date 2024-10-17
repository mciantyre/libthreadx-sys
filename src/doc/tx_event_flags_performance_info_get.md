Get event flags group performance information

### Description

This service retrieves performance information about the specified event flags group.

> **Important:** *ThreadX library and application must be built with* **TX_EVENT_FLAGS_ENABLE_PERFORMANCE_INFO*: *defined for this service to return performance information.*

### Parameters

- *group_ptr*: Pointer to previously created event flags group.
- *sets*: Pointer to destination for the number of event flags set requests performed on this group.
- *gets*: Pointer to destination for the number of event flags get requests performed on this group.
- *suspensions*: Pointer to destination for the number of thread event flags get suspensions on this group.
- *timeouts*: Pointer to destination for the number of event flags get suspension timeouts on this group.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful event flags group performance get.
- **TX_PTR_ERROR** (0x03) Invalid event flags group pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_delete`]
- [`tx_event_flags_get`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]
