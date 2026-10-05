pub open spec fn clock_rate_notify__3_6_2_12_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_is_invalid(old_s, clock_id))
    && (result == INVALID_PARAMETERS ==> (notify_enable != 0 || (notify_enable & 0x1) != (notify_enable & 0x1)))
    && (result == SUCCESS ==> (notify_enable == 0 || notify_enable == 1))
    && (result == SUCCESS ==> reserved_bits_are_zero(old_s, notify_enable))
}

fn clock_id_is_invalid(old_s: S, clock_id: uint32) -> bool {
    clock_id == 0
}

fn reserved_bits_are_zero(old_s: S, notify_enable: uint32) -> bool {
    (notify_enable & 0xFFFFFFFE) == 0
}