Deactivate application timer

### Description

This service deactivates the specified application timer. If the timer is already deactivated, this service has no effect.

### Parameters

- *timer_ptr*: Pointer to a previously created application timer.

### Return Values

- **TX_SUCCESS** (0x00) Successful application timer deactivation.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_timer_activate`]
- [`tx_timer_change`]
- [`tx_timer_create`]
- [`tx_timer_delete`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

