Set event flags in an event flags group

### Description

This service sets or clears event flags in an event flags group, depending upon the specified set-option. All suspended threads whose event flags request is now satisfied are resumed.

### Parameters

- *group_ptr*: <br>Pointer to the previously created event flags group control block.
- *flags_to_set*: <br>Specifies the event flags to set or clear based upon the set option selected.
- *set_option*: <br>Specifies whether the event flags specified are ANDed or ORed into the current event flags of the group. The following are valid selections:
  - **TX_AND** (0x02)
  - **TX_OR** (0x00)

  Selecting TX_AND specifies that the specified event flags are **AND**ed into the current event flags in the group. This option is often used to clear event flags in a group. Otherwise, if TX_OR is specified, the specified event flags are **OR**ed with the current event in the group.

### Return Values
- **TX_SUCCESS** (0x00) Successful event flags set.
- **TX_GROUP_ERROR** (0x06) Invalid pointer to event flags group.
- **TX_OPTION_ERROR** (0x08) Invalid set-option specified.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

Yes

### See Also

- [`tx_event_flags_create`]
- [`tx_event_flags_delete`]
- [`tx_event_flags_get`]
- [`tx_event_flags_info_get`]
- [`tx_event_flags_performance_info_get`]
- [`tx_event_flags_performance_system_info_get`]
- [`tx_event_flags_set_notify`]

