Get timer performance information

### Description

This service retrieves performance information about the specified application timer.

> **Important:** *The ThreadX library and application must be built with* ***TX_TIMER_ENABLE_PERFORMANCE_INFO*** *defined for this service to return performance information.*

### Parameters
- *timer_ptr*: Pointer to previously created timer.
- *activates*: Pointer to destination for the number of activation requests performed on this timer.
- *reactivates*: Pointer to destination for the number of automatic reactivations performed on this periodic timer.
- *deactivates*: Pointer to destination for the number of deactivation requests performed on this timer.
- *expirations*: Pointer to destination for the number of expirations of this timer.
- *expiration_adjusts*: Pointer to destination for the number of internal expiration adjustments performed on this timer. These adjustments are done in the timer interrupt processing for timers that are larger than the default timer list size (by default timers with expirations greater than 32 ticks).

> **Note:** *Supplying a TX_NULL for any parameter indicates the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful timer performance get.
- **TX_PTR_ERROR** (0x03) Invalid timer pointer.
- **TX_FEATURE_NOT_ENABLED** (0xFF) The system was not compiled with performance information enabled.

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
- [`tx_timer_performance_system_info_get`]

