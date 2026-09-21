pub open spec fn ffa_notification_bitmap_create_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == INVALID_PARAMETERS ==> (old_s.vm_id != new_s.vm_id || (old_s.vm_id as int) < 0 || (old_s.vm_id as int) >= (1u64 << 32)))
    && (result == NOT_SUPPORTED ==> true)
    && (result == DENIED ==> (old_s.notification_bitmap_exists(new_s.vm_id) && new_s.notification_bitmap_exists(new_s.vm_id)))
    && (result == NO_MEMORY ==> (old_s.notification_bitmap_exists(new_s.vm_id) || !new_s.notification_bitmap_exists(new_s.vm_id)))
    && (result == 0 ==> (old_s.vm_id == new_s.vm_id && (old_s.vm_id as int) >= 0 && (old_s.vm_id as int) < (1u64 << 32) && !old_s.notification_bitmap_exists(new_s.vm_id) && new_s.notification_bitmap_exists(new_s.vm_id)))
}