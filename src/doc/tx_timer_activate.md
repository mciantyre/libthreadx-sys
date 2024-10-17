Activate application timer

### Description

This service activates the specified application timer. The expiration routines of timers that expire at the same time are executed in the order they were activated.

> **Note:** *An expired one-shot timer must be reset via* ***tx_timer_change*** *before it can be activated again.*

### Parameters

- *timer_ptr*: Pointer to a previously created application timer.

### Return Values

- **TX_SUCCESS** (0x00) Successful application timer activation.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer.
- **TX_ACTIVATE_ERROR** (0x17) Timer was already active or is a one-shot timer that has already expired.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_timer_change`]
- [`tx_timer_create`]
- [`tx_timer_deactivate`]
- [`tx_timer_delete`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

