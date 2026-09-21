pub open spec fn ffa_notification_bitmap_create_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.vm_id != new_s.vm_id))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> (old_s.vm_id == new_s.vm_id))
    && (result == FFA_NO_MEMORY ==> true)
    && (result == FFA_SUCCESS ==> (old_s.vm_id == new_s.vm_id))
}