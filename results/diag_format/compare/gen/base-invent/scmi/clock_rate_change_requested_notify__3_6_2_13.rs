pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_invalid(old_s, clock_id))
    && (result == SUCCESS ==> (notify_enable == 0 || notify_enable == 1))
    && (result == SUCCESS ==> reserved_bits == 0)
}

fn clock_id_invalid(old_s: S, clock_id: uint32) -> bool {
    true
}

fn reserved_bits == 0 -> bool {
    (notify_enable >> 1) == 0
}