pub open spec fn telemetry_config_set__3_12_4_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure conditions
    // control[31:9] must be zero
    && (control_bits_31_9(old_s) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // control[4:1] must be 0, 1, or 2
    && (control_bits_4_1(old_s) != 0 && control_bits_4_1(old_s) != 1 && control_bits_4_1(old_s) != 2 ==> ResultEqual(result, INVALID_PARAMETERS))
    // control[0] == 0 implies control[4:1] is ignored (no specific failure, but if control[0]==0 and control[4:1] is invalid, it's still invalid per above)
    // control[0] == 0 implies sampling_rate is ignored (no specific failure)
    // control[0] == 0 implies group_identifier is ignored if control[8:5] == 0 (no specific failure)
    // control[8:5] == 2 implies group_identifier is ignored (no specific failure)
    // control[8:5] == 0 implies command applies to all DEs not associated with any event group
    // control[8:5] == 1 implies command applies to DEs in specified event group only
    // control[8:5] == 2 implies command applies to all DEs and all event groups
    // control[0] == 1 enables telemetry collection
    // control[0] == 0 disables telemetry collection
    // sampling_rate constraints:
    //   - Bits[31:21] must be zero
    //   - Bits[4:0] is exponent (two's complement)
    //   - Bits[20:5] is sec
    //   - If control[0] == 0 OR control[4:1] == 2, sampling_rate is ignored (no specific failure)
    //   - Otherwise, sampling_rate must be valid (non-zero, within range)
    //   - If sampling_rate is invalid (e.g., zero or out of range), return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled
    //   - If control[8:5] == 0, command applies to all DEs not associated with any event group
    //   - If control[8:5] == 1, command applies to DEs in specified event group only
    //   - If control[8:5] == 2, command applies to all DEs and all event groups
    //   - If control[0] == 0, sampling_rate is ignored
    //   - If control[0] == 1, sampling_rate must be valid
    //   - If control[4:1] == 2, sampling_rate is ignored
    //   - If control[4:1] != 2, sampling_rate must be valid
    //   - If sampling_rate is invalid, return OUT_OF_RANGE
    //   - If sampling_rate is valid, return SUCCESS
    //   - If no DE has been enabled, return INVALID_PARAMETERS
    //   - If number of DEs or event groups enabled has reached a maximum limit, return OUT_OF_RANGE
    //   - If control[0] == 0, telemetry is disabled (no specific failure, but if no DEs are enabled, it's INVALID_PARAMETERS)
    //   - If control[0] == 1, telemetry is enabled