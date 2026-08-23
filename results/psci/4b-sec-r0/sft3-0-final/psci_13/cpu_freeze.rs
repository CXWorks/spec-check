pub open spec fn cpu_freeze_spec(target_cpu: Bits64, old_s: S, new_s: S) -> bool {
  (CpuIsOn(old_s, target_cpu) ==> result == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> CpuIsOn(new_s, target_cpu))
  && (result.is_Ok() ==> CpuIsOnPending(new_s, target_cpu))
  && ((!(CpuIsOn(old_s, target_cpu)) &&
       !(CpuIsOnPending(old_s, target_cpu)))
    ==> CpuIsOn(new_s, target_cpu))
  && ((!(CpuIsOn(old_s, target_cpu)) &&
       !(CpuIsOnPending(old_s, target_cpu)))
    ==> CpuIsOnPending(new_s, target_cpu))
}