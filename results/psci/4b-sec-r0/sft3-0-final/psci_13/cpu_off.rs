pub open spec fn cpu_off_spec(target_cpu: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (CpuIsOn(old_s, target_cpu as Bits64) ==> ResultEqual(result, RmiError(RmiErrorKind::DENIED)))
  && (result.is_Ok() ==> CpuIsOn(new_s, target_cpu as Bits64) == false)
  && ((!(CpuIsOn(old_s, target_cpu as Bits64)))
    ==> CpuIsOn(new_s, target_cpu as Bits64) == CpuIsOn(old_s, target_cpu as Bits64))
}