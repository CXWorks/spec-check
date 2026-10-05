pub open spec fn sbi_debug_update_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (Exists(i, 0 <= i < trig_count(old_s) && !IsInstalledTrigger(TrigConfig(old_s, i).trig_idx)) ==> CommandFailed(result))
    && (Exists(i, 0 <= i < trig_count(old_s) && TrigConfig(old_s, i).trig_tdata1.type != InstalledTrigger(TrigConfig(old_s, i).trig_idx).tdata1.type) ==> CommandFailed(result))
    && (Exists(i, 0 <= i < trig_count(old_s) && TrigConfig(old_s, i).trig_tdata1.chain != InstalledTrigger(TrigConfig(old_s, i).trig_idx).tdata1.chain) ==> CommandFailed(result))
    && (CommandSucceeded(result) ==> ForAll(i, 0 <= i < trig_count(old_s) => InstalledTrigger(new_s, TrigConfig(old_s, i).trig_idx) is updated based on TrigConfig(old_s, i)))
}