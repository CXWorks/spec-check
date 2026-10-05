pub open spec fn sbi_debug_update_triggers_spec(trig_count: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (exists|i: UInt64| i < trig_count && !IsInstalledDebugTrigger(new_s, ShmemTrigIdx(i)) ==> CallFails(result))
  && (exists|i: UInt64| i < trig_count && TriggerType(new_s, ShmemTrigTdata1(i)) != InstalledDebugTrigger(new_s, ShmemTrigIdx(i)).type ==> CallFails(result))
  && (exists|i: UInt64| i < trig_count && TriggerChain(new_s, ShmemTrigTdata1(i)) != InstalledDebugTrigger(new_s, ShmemTrigIdx(i)).chain ==> CallFails(result))
  && (CallSucceeds(result) ==> forall|i: UInt64| i < trig_count ==> DebugTriggerUpdatedFrom(new_s, ShmemTrigIdx(i), ShmemTrigTdata1(i), ShmemTrigTdata2(i), ShmemTrigTdata3(i)))
  && (CallSucceeds(result) ==> CallSucceeds(result))
  && ((!(exists|i: UInt64| i < trig_count && !IsInstalledDebugTrigger(old_s, ShmemTrigIdx(i))) &&
       !(exists|i: UInt64| i < trig_count && TriggerType(old_s, ShmemTrigTdata1(i)) != InstalledDebugTrigger(old_s, ShmemTrigIdx(i)).type) &&
       !(exists|i: UInt64| i < trig_count && TriggerChain(old_s, ShmemTrigTdata1(i)) != InstalledDebugTrigger(old_s, ShmemTrigIdx(i)).chain))
    ==> CallSucceeds(result))
  && (CallFails(result)
    ==> forall|i: UInt64| i < trig_count ==> !(DebugTriggerUpdatedFrom(new_s, ShmemTrigIdx(i), ShmemTrigTdata1(i), ShmemTrigTdata2(i), ShmemTrigTdata3(i))))
}