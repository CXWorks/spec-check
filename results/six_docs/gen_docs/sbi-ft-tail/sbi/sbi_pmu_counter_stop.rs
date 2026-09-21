pub open spec fn sbi_pmu_counter_stop_spec(counter_idx_base: UInt64, counter_idx_mask: UInt64, stop_flags: UInt64, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> true)
  && (result == SBI_ERR_ALREADY_STOPPED ==> true)
  && (result == SBI_ERR_NO_SHMEM ==> true)
  && (result == SBI_SUCCESS ==> true)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       !(result == SBI_ERR_ALREADY_STOPPED) &&
       !(result == SBI_ERR_NO_SHMEM))
    ==> result == SBI_SUCCESS)
}