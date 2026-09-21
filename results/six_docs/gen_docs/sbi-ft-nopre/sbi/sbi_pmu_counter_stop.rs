pub open spec fn sbi_pmu_counter_stop_spec(counter_idx_base: UInt64, counter_idx_mask: UInt64, stop_flags: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> (counter_idx_mask & 3) == 0)
  && (result == SBI_ERR_INVALID_PARAM ==> (stop_flags & SBI_PMU_STOP_FLAG_RESET) == SBI_PMU_STOP_FLAG_RESET)
  && (result == SBI_ERR_INVALID_PARAM ==> (stop_flags & SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT) == SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT)
  && (result == SBI_ERR_INVALID_PARAM ==> (stop_flags & 3) == 0)
  && (result == SBI_ERR_ALREADY_STOPPED ==> (counter_idx_mask & 3) == 0)
  && (result == SBI_ERR_ALREADY_STOPPED ==> (stop_flags & SBI_PMU_STOP_FLAG_RESET) == SBI_PMU_STOP_FLAG_RESET)
  && (result == SBI_ERR_ALREADY_STOPPED ==> (stop_flags & SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT) == SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT)
  && (result == SBI_ERR_ALREADY_STOPPED ==> (stop_flags & 3) == 0)
  && (result == SBI_ERR_NO_SHMEM ==> (counter_idx_mask & 3) == 0)
  && (result == SBI_ERR_NO_SHMEM ==> (stop_flags & SBI_PMU_STOP_FLAG_RESET) == SBI_PMU_STOP_FLAG_RESET)
  && (result == SBI_ERR_NO_SHMEM ==> (stop_flags & SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT) == SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT)
  && (result == SBI_ERR_NO_SHMEM ==> (stop_flags & 3) == 0)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       !(result == SBI_ERR_ALREADY_STOPPED) &&
       !(result == SBI_ERR_NO_SHMEM))
    ==> result == SBI_SUCCESS)
}