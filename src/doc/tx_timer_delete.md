Delete application timer

### Description

This service deletes the specified application timer.

> **Note:** *It is the application's responsibility to prevent use of a deleted timer.*

### Parameters

- *timer_ptr*: Pointer to a previously created application timer.

### Return Values

- **TX_SUCCESS** (0x00) Successful application timer deletion.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

### See Also

- [`tx_timer_activate`]
- [`tx_timer_change`]
- [`tx_timer_create`]
- [`tx_timer_deactivate`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

