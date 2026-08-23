pub open spec fn affinity_info_spec(affinity_level: UInt32, target_affinity: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> (CpuIsOn(new_s, (affinity_level as int) + 1)))
  && (result.is_Ok() ==> (CpuIsOn(new_s, (affinity_level as int) + 1)))
}