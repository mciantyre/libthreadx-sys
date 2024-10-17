Enable and disable interrupts

### Description

This service enables or disables interrupts as specified by the input parameter *new_posture*.

> **Note:** *If this service is called from an application thread, the interrupt posture remains part of that thread's context. For example, if the thread calls this routine to disable interrupts and then suspends, when it is resumed, interrupts are disabled again.*

> **Warning:** *This service should not be used to enable interrupts during initialization! Doing so could cause unpredictable results.*

### Parameters

- *new_posture*: This parameter specifies whether interrupts are disabled or enabled. Legal values include **TX_INT_DISABLE** and **TX_INT_ENABLE**. The actual values for these parameters are port specific. In addition, some processing architectures might support additional interrupt disable postures.

### Return Values
- **previous posture** This service returns the previous interrupt posture to the caller. This allows users of the service to restore the previous posture after interrupts are disabled.

### Allowed From

Threads, timers, and ISRs

### Preemption Possible

No

### See Also

None

