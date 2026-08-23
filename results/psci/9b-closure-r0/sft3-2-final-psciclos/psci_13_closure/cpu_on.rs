pub open spec fn cpu_on_spec(target_cpu: UInt32, entry_point_address: UInt32, context_id: UInt32, result: int, old_s: S, new_s: S) -> bool {
  (result == INVALID_PARAMETERS ==> CpuIsValid(old_s, target_cpu as Bits64))
  && (result == INVALID_ADDRESS ==> AddrIsNonSecure(old_s, entry_point_address as Address))
  && (result == ALREADY_ON ==> CpuIsOn(old_s, target_cpu as Bits64))
  && (result == ON_PENDING ==> CpuIsOnPending(old_s, target_cpu as Bits64))
  && ((!(CpuIsValid(old_s, target_cpu as Bits64)) &&
       AddrIsNonSecure(old_s, entry_point_address as Address) &&
       !(CpuIsOn(old_s, target_cpu as Bits64)) &&
       !(CpuIsOnPending(old_s, target_cpu as Bits64)))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> CpuIsOn(new_s, target_cpu as Bits64) == CpuIsOn(old_s, target_cpu as Bits64))
  && (result != SUCCESS
    ==> CpuIsOnPending(new_s, target_cpu as Bits64) == CpuIsOnPending(old_s, target_cpu as Bits64))
}