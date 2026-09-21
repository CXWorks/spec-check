pub open spec fn 3.12.4.9_telemetry_de_enabled_list_spec(result: int, flags: uint32, array: {uint32, uint32}, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> flags == 0)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_SUCCESS ==> (flags >= 0) && (array.len() == (flags as int) & 0xFFFF))
}