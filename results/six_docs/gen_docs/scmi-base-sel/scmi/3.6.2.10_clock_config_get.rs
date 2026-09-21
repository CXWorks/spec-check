pub open spec fn clock_config_get_spec(result: int, attributes: UInt32, config: UInt32, extended_config_val: UInt32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (attributes & 0xFF == 0 || (attributes & 0xFF) != 0))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_SUCCESS ==> (attributes & 0xFF == 0 || (attributes & 0xFF) != 0) && (config & 0xFFFFFFFE == 0) && (extended_config_val == 0 || (attributes & 0xFF) != 0))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_INVALID_PARAMETERS ==> true)
}