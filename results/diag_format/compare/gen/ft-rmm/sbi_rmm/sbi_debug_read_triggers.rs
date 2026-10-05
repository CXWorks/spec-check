pub open spec fn sbi_debug_read_triggers_spec(trig_idx_base: unsigned long, trig_count: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (result.success ==> forall i : 0 <= i < trig_count : SharedMemWords(new_s, offset = i * (XLEN / 2), count = 4, width = XLEN, endian = LITTLE) == DebugTriggerStateAndConfig(new_s, CurrentHart(new_s), trig_idx_base + i))
}