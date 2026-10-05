pub open spec fn ffa_notification_bitmap_create_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == INVALID_PARAMETERS ==> (old_s.vm_id != new_s.vm_id || !old_s.vm_id_is_valid(old_s.vm_id)))
    && (result == NOT_SUPPORTED ==> true)
    && (result == DENIED ==> old_s.notification_bitmap_created(new_s.vm_id))
    && (result == NO_MEMORY ==> true)
    && (result == FFA_SUCCESS ==> (new_s.notification_bitmap_created(new_s.vm_id) && new_s.notification_count(new_s.vm_id) == (old_s.notification_count(new_s.vm_id) as int) + 64))
}