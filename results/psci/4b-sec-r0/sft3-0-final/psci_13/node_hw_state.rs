pub open spec fn node_hw_state_spec(target_cpu: UInt32, power_level: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> (CpuIsOn(new_s, target_cpu as int)))
  && (!result.is_Err() ==> (CpuIsOn(new_s, target_cpu as int)))
}