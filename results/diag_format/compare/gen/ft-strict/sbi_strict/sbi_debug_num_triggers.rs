pub open spec fn sbi_debug_num_triggers_spec(trig_tdata1: UInt64, result: Result<Int64, UInt64>, old_s: S, new_s: S) -> bool {
  (result.value == NumTriggers(new_s, trig_tdata1))
}