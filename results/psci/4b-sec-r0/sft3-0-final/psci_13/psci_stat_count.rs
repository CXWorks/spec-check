pub open spec fn psci_stat_count_spec(target_cpu: UInt32, power_state: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> (CpuIsOn(new_s, target_cpu as Bits64)))
  && (result.is_Ok() ==> (CpuIsOnPending(new_s, target_cpu as Bits64)))
  && (result.is_Err()
    ==> CpuIsOn(new_s, target_cpu as Bits64))
  && (result.is_Err()
    ==> CpuIsOnPending(new_s, target_cpu as Bits64))
}