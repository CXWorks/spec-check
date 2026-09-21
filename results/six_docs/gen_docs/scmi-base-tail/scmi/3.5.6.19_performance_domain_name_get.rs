pub open spec fn 3.5.6.19_performance_domain_name_get_spec(result: int32, name: [u8; 64], flags: u32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_NOT_FOUND ==> (flags == 0 && name[63] == 0))
    && (result != SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
}