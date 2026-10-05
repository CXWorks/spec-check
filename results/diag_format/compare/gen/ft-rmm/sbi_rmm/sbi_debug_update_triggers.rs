pub open spec fn sbi_debug_update_triggers_spec(trig_count: unsigned long, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (Exists(i, 0 <= i < trig_count, !IsInstalledTrigger(TrigConfig(i).trig_idx)) ==> CommandFailed(result))
  && (Exists(i, 0 <= i < trig_count, TrigConfig(i).trig_tdata1.type != InstalledTrigger(TrigConfig(i).trig_idx).tdata1.type) ==> CommandFailed(result))
  && (Exists(i, 0 <= i < trig_count, TrigConfig(i).trig_tdata1.chain != InstalledTrigger(TrigConfig(i).trig_idx).tdata1.chain) ==> CommandFailed(result))
  && (result == SBI_SUCCESS ==> ForAll(i, 0 <= i < trig_count, InstalledTrigger(TrigConfig(i).trig_idx) is updated based on TrigConfig(i)))
  && ((!(Exists(i, 0 <= i < trig_count, !IsInstalledTrigger(TrigConfig(i).trig_idx))) &&
       !(Exists(i, 0 <= i < trig_count, TrigConfig(i).trig_tdata1.type != InstalledTrigger(TrigConfig(i).trig_idx).tdata1.type)) &&
       !(Exists(i, 0 <= i < trig_count, TrigConfig(i).trig_tdata1.chain != InstalledTrigger(TrigConfig(i).trig_idx).tdata1.chain)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> ForAll(i, 0 <= i < trig_count, InstalledTrigger(TrigConfig(i).trig_idx) == InstalledTrigger(TrigConfig(i).trig_idx)))
}