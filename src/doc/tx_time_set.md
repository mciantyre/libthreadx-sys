Sets the current time

### Description

This service sets the internal system clock to the specified value. Each timer-tick increases the internal system clock by one.

> **Note:** *The actual time each timer-tick represents is application specific.*

### Parameters

- *new_time*: New time to put in the system clock, legal values range from 0 through 0xFFFFFFFF.

### Return Values

None

### Allowed From

Threads, timers, and ISRs

### Preemption Possible

No

### See Also

- [`tx_time_get`]

