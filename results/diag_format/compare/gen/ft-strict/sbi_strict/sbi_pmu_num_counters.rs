pub open spec fn sbi_pmu_num_counters_spec(error: sbiret, value: UInt64, old_s: S, new_s: S) -> bool {
  (ResultEqual(error, SBI_SUCCESS) ==> value == NumHardwareCounters() + NumFirmwareCounters())
}