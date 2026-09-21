pub open spec fn 3.3.2.10_power_domain_name_get_spec(result: int, flags: u64, ext_name: [u8; 64], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (flags == 0 && ext_name[63] == 0))
    && (result == SCMI_NOT_FOUND ==> (flags == 0 && ext_name[63] == 0))
    && (result != SCMI_SUCCESS ==> (flags == 0 && ext_name[63] == 0))
}