Retrieve performance system information

### Description

This service retrieves performance information about all event flags groups in the system.

> **Important:** *ThreadX library and application must be built with* **TX_EVENT_FLAGS_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *sets*: Pointer to destination for the total number of event flags set requests performed on all groups.
- *gets*: Pointer to destination for the total number of event flags get requests performed on all groups.
- *suspensions*: Pointer to destination for the total number of thread event flags get suspensions on all groups.
- *timeouts*: Pointer to destination for the total number of event flags get suspension timeouts on all groups.

> **Note:** *Supplying a TX_NULL for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful event flags system performance get.
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
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_set`]
- [`tx_event_flags_set_notify`]

