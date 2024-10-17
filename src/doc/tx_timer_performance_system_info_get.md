Get timer system performance information

### Description

This service retrieves performance information about all the application timers in the system.

> **Important:** *The ThreadX library and application must be built with* **TX_TIMER_ENABLE_PERFORMANCE_INFO** *defined for this service to return performance information.*

### Parameters

- *activates*: Pointer to destination for the total number of activation requests performed on all timers.
- *reactivates*: Pointer to destination for the total number of automatic reactivation performed on all periodic timers.
- *deactivates*: Pointer to destination for the total number of deactivation requests performed on all timers.
- *expirations*: Pointer to destination for the total number of expirations on all timers.
- *expiration_adjusts*: Pointer to destination for the total number of internal expiration adjustments performed on all timers. These adjustments are done in the timer interrupt processing for timers that are larger than the default timer list size (by default timers with expirations greater than 32 ticks).

> **Note:** *Supplying a **TX_NULL** for any parameter indicates that the parameter is not required.*

### Return Values

- **TX_SUCCESS** (0x00) Successful timer system performance get.
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
- [`tx_timer_performance_info_get`]
