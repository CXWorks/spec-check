pub open spec fn ffa_notification_bitmap_destroy_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.vm_id as int) < 0)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_SUCCESS ==> true)
}