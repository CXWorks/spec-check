pub open spec fn cpu_default_suspend_spec(entry_point_address: UInt64, context_id: UInt32, result: Result<(), int>, old_s: S, new_s: S) -> bool {
  (result == INVALID_ADDRESS ==> AddrIsNonSecure(new_s, entry_point_address))
  && ((result == SUCCESS || result == INVALID_ADDRESS) ==> CpuIsOn(new_s, 0))
  && ((result == SUCCESS || result == INVALID_ADDRESS) ==> CpuIsOnPending(new_s, 0))
  && ((result == SUCCESS || result == INVALID_ADDRESS) ==> CpuIsOnPending(new_s, 0))
}