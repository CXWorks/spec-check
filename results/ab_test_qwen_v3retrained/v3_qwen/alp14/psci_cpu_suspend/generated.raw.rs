pub open spec fn psci_cpu_suspend_spec(fid: UInt64, power_state: UInt32, entry_point_address: Address, context_id: UInt64, result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RMI_SUCCESS ==> false)
  && (result != RMI_SUCCESS ==> true)
}