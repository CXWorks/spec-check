pub open spec fn ffa_notification_get_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.cmd_input_flags & 0x1 != 0 || old_s.cmd_input_flags & 0x2 != 0 || old_s.cmd_input_flags & 0x4 != 0 || old_s.cmd_input_flags & 0x8 != 0))
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> (old_s.cmd_input_flags & 0x1 == 0 || (new_s.sp_notifications_bitmap == old_s.sp_notifications_bitmap && (old_s.cmd_input_flags & 0x1 != 0)))
        && (old_s.cmd_input_flags & 0x2 == 0 || (new_s.vm_notifications_bitmap == old_s.vm_notifications_bitmap && (old_s.cmd_input_flags & 0x2 != 0)))
        && (old_s.cmd_input_flags & 0x4 == 0 || (new_s.spm_framework_notifications_bitmap == old_s.spm_framework_notifications_bitmap && (old_s.cmd_input_flags & 0x4 != 0)))
        && (old_s.cmd_input_flags & 0x8 == 0 || (new_s.hypervisor_framework_notifications_bitmap == old_s.hypervisor_framework_notifications_bitmap && (old_s.cmd_input_flags & 0x8 != 0))))
}