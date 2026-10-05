pub open spec fn sbi_debug_update_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (forall i: UInt64 | i < trig_count(old_s) ==> !IsInstalledDebugTrigger(ShmemTrigIdx(i, old_s))) ==> CallFails(result)
    && (forall i: UInt64 | i < trig_count(old_s) ==> TriggerType(ShmemTrigTdata1(i, old_s)) != InstalledDebugTrigger(ShmemTrigIdx(i, old_s)).type) ==> CallFails(result)
    && (forall i: UInt64 | i < trig_count(old_s) ==> TriggerChain(ShmemTrigTdata1(i, old_s)) != InstalledDebugTrigger(ShmemTrigIdx(i, old_s)).chain) ==> CallFails(result)
    && (forall i: UInt64 | i < trig_count(old_s) ==> DebugTriggerUpdatedFrom(ShmemTrigIdx(i, old_s), ShmemTrigTdata1(i, old_s), ShmemTrigTdata2(i, old_s), ShmemTrigTdata3(i, old_s))) ==> CallSucceeds(result)
    && (forall i: UInt64 | i < trig_count(old_s) ==> InstalledDebugTrigger(ShmemTrigIdx(i, old_s))) ==> CallSucceeds(result)
}