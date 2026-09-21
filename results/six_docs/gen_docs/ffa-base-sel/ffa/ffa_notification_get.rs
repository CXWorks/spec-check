pub open spec fn ffa_notification_get_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.receiver_id as int < 0 || old_s.receiver_id as int >= (1u32 << 32) || (old_s.flags as int) & 0xF != 0))
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> (old_s.receiver_id as int >= 0 && old_s.receiver_id as int < (1u32 << 32) && ((old_s.flags as int) & 0xF) == 0))
}