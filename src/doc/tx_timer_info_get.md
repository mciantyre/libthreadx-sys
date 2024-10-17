Retrieve information about an application timer

### Description

This service retrieves information about the specified application timer.

### Parameters

- *timer_ptr*: Pointer to a previously created application timer.
- *name*: Pointer to destination for the pointer to the timer's name.
- *active*: Pointer to destination for the timer active indication. If the timer is inactive or this service is called from the timer itself, a **TX_FALSE** value is returned. Otherwise, if the timer is active, a **TX_TRUE** value is returned.
- *remaining_ticks*: Pointer to destination for the number of timer ticks left before the timer expires.
- *reschedule_ticks*: Pointer to destination for the number of timer ticks that will be used to automatically reschedule this timer. If the value is zero, then the timer is a one-shot and won't be rescheduled.
- *next_timer*: Pointer to destination for the pointer of the next created application timer.

> **Note:** *Supplying a **TX_NULL** for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful timer information retrieval.
- **TX_TIMER_ERROR** (0x15) Invalid application timer pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_timer_activate`]
- [`tx_timer_change`]
- [`tx_timer_create`]
- [`tx_timer_deactivate`]
- [`tx_timer_delete`]
- [`tx_timer_info_get`]
- [`tx_timer_performance_info_get`]
- [`tx_timer_performance_system_info_get`]

