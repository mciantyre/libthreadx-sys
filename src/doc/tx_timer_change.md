Change application timer

### Description

This service changes the expiration characteristics of the specified application timer. The timer must be deactivated prior to calling this service.

> **Note:** *A call to the* ***tx_timer_activate*** *service is required after this service in order to start the timer again.*

### Parameters

- *timer_ptr*: Pointer to a timer control block.
- *initial_ticks*: Specifies the initial number of ticks for timer expiration. Legal values range from 1 through 0xFFFFFFFF.
- *reschedule_ticks*: Specifies the number of ticks for all timer expirations after the first. A zero for this parameter makes the timer a *one-shot* timer. Otherwise, for periodic timers, legal values range from 1 through 0xFFFFFFFF.

> **Note:** *An expired one-shot timer must be reset via*
***tx_timer_change*** *before it can be activated again.*

### Return Values

- **TX_SUCCESS** (0x00) Successful application timer change.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer.
- **TX_TICK_ERROR** (0x16) Invalid value (a zero) supplied for initial ticks.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_timer_activate`]
- [`tx_timer_create`]
- [`tx_timer_deactivate`]
- [`tx_timer_delete`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

