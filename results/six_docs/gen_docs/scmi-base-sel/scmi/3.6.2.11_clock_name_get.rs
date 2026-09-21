pub open spec fn 3.6.2.11_clock_name_get_spec(result: int, flags: u64, name: [u8; 64], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_NOT_FOUND ==> (flags == 0 && name[63] == 0))
    && (result != SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_SUCCESS ==> (name[0..63] as [u8; 64] == old_s.name))
    && (result == SCMI_SUCCESS ==> (new_s.name == old_s.name))
    && (result == SCMI_SUCCESS ==> (new_s.flags == old_s.flags))
    && (result == SCMI_SUCCESS ==> (new_s.clock_id == old_s.clock_id))
}