pub open spec fn migrate_spec(target_cpu: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> CpuIsOn(new_s, target_cpu))
  && ((!(CpuIsValid(old_s, target_cpu)) ||
       CpuIsOn(old_s, target_cpu))
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS
    ==> CpuIsOnPending(new_s, target_cpu))
  && ((!(CpuIsValid(old_s, target_cpu)))
    ==> result == RSI_ERROR_INPUT)
  && (result != RSI_SUCCESS
    ==> CpuIsOn(new_s, target_cpu) == CpuIsOn(old_s, target_cpu))
  && (result != RSI_SUCCESS
    ==> CpuIsOnPending(new_s, target_cpu) == CpuIsOnPending(old_s, target_cpu))
}