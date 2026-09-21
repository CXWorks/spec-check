pub open spec fn ffa_notification_get2_spec(receiver_id: uint32, flags: uint64, sp_notification_bitmask: [uint64; 1], vm_notification_bitmask: [uint64; 1], spmc_framework_notification_bitmask: uint64, hypervisor_framework_notification_bitmask: uint64, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  true
}