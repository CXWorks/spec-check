pub open spec fn ffa_notification_bitmap_destroy_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.vm_id != old_s.vm_id))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> true)
    && (result == FFA_ERROR_DENIED ==> true)
    && (result == FFA_SUCCESS ==> true)
}