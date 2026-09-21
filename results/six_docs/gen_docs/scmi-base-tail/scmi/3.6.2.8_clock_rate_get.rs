pub open spec fn clock_rate_get_spec(result: int32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (rate[0] == 0 || rate[1] == 0 || rate[0] == 0 || rate[1] == 0))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> true)
}