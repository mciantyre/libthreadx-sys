Create application timer

### Description

This service creates an application timer with the specified
expiration function and
periodic.

### Parameters

- *timer_ptr*: Pointer to a timer control block
- *name_ptr*: Pointer to the name of the timer.
- *expiration_function*: Application function to call when the timer expires.
- *expiration_input*: Input to pass to expiration function when timer expires.
- *initial_ticks*: Specifies the initial number of ticks for timer expiration. Legal values range from 1 through 0xFFFFFFFF.
- *reschedule_ticks*: Specifies the number of ticks for all timer expirations after the first. A zero for this parameter makes the timer a *one-shot* timer. Otherwise, for periodic timers, legal values range from 1 through 0xFFFFFFFF.

  > **Note:** *After a one-shot timer expires, it must be reset via   tx_timer_change before it can be activated again.*

- *auto_activate*: Determines if the timer is automatically activated during creation. If this value is **TX_AUTO_ACTIVATE** (0x01) the timer is made active. Otherwise, if the value **TX_NO_ACTIVATE** (0x00) is selected, the timer is created in a non-active state. In this case, a subsequent ***tx_timer_activate*** service call is necessary to get the timer actually started.

### Return Values

- **TX_SUCCESS** (0x00) Successful application timer creation.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer. Either the pointer is NULL or the timer is already created.
- **TX_TICK_ERROR** (0x16) Invalid value (a zero) supplied for initial ticks.
- **TX_ACTIVATE_ERROR** (0x17) Invalid activation selected.
- **TX_CALLER_ERROR** (0x13) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

### See Also

- [`tx_timer_activate`]
- [`tx_timer_change`]
- [`tx_timer_deactivate`]
- [`tx_timer_delete`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

