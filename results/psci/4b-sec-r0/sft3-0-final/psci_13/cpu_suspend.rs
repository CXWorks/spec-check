pub open spec fn cpu_suspend_spec(target_cpu: UInt32, power_state: UInt32, entry_point_address: Address, context_id: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> (CpuIsOn(new_s, target_cpu as int)))
  && ((!(result.is_Err()) &&
       (power_state & 0x3f) == 0)
    ==> CpuIsOn(new_s, target_cpu as int))
  && (result.is_Err()
    ==> CpuIsOn(new_s, target_cpu as int))
  && (result.is_Ok()
    ==> CpuIsOn(new_s, target_cpu as int))
}