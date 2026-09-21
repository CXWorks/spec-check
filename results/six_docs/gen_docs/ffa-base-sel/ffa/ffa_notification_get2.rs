pub open spec fn ffa_notification_get2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.receiver_id as int) < 0
        || (old_s.flags as int) < 0
        || (old_s.sp_notification_bitmask as int) < 0
        || (old_s.vm_notification_bitmask as int) < 0
        || (old_s.spmc_notification_bitmask as int) < 0
        || (old_s.hypervisor_notification_bitmask as int) < 0
    ))
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}