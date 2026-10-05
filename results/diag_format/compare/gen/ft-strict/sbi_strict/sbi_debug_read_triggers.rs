pub open spec fn sbi_debug_read_triggers_spec(trig_idx_base: unsigned long, trig_count: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (result.ret == 0 ==> (forall (i: UInt64), (i < trig_count) ==> SharedMemoryHoldsTriggerStateLE(new_s, CallingHart(), trig_idx_base + i, i * (XLEN / 2) as int, 4)))
}