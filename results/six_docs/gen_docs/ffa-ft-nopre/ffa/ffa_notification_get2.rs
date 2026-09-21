pub open spec fn ffa_notification_get2_spec(receiver_id: UInt32, flags: UInt64, sp_notification_bitmask: [UInt64; 1], vm_notification_bitmask: [UInt64; 1], spmc_framework_notification_bitmask: UInt64, hypervisor_framework_notification_bitmask: UInt64, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  true
}