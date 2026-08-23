pub open spec fn affinity_info_spec(target_affinity: UInt64, lowest_affinity_level: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> (result == RSI_SUCCESS))
  && ((!(CpuIsValid(old_s, 0)) &&
       !(CpuIsValid(old_s, 1)) &&
       !(CpuIsValid(old_s, 2)) &&
       !(CpuIsValid(old_s, 3)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CpuIsValid(new_s, 0) == CpuIsValid(old_s, 0))
  && (result != RSI_SUCCESS
    ==> CpuIsValid(new_s, 1) == CpuIsValid(old_s, 1))
  && (result != RSI_SUCCESS
    ==> CpuIsValid(new_s, 2) == CpuIsValid(old_s, 2))
  && (result != RSI_SUCCESS
    ==> CpuIsValid(new_s, 3) == CpuIsValid(old_s, 3))
}