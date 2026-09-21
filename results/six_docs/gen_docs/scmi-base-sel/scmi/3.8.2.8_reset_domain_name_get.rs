pub open spec fn 3.8.2.8_reset_domain_name_get_spec(result: int, flags: u64, name: [u8; 64], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_NOT_FOUND ==> (flags == 0 && name[63] == 0))
    && (result != SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
}