Retrieves the current time

Application Timers

### Description

This service returns the contents of the internal system clock. Each timertick increases the internal system clock by one. The system clock is set to zero during initialization and can be changed to a specific value by the service ***tx_time_set***.

> **Note:** *The actual time each timer-tick represents is application specific.*

### Parameters

None

### Return Values

- *system clock ticks** Value of the internal, free running, system clock.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible
No

### See Also

- [`tx_time_set`]

