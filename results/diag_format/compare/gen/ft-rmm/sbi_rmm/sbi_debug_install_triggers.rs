pub open spec fn sbi_debug_install_triggers_spec(trig_count: unsigned long, old_s: S, new_s: S) -> bool {
  TriggerConfigsProcessedInIncreasingIndexOrder(new_s, 0, trig_count)
  && (forall i in [0, trig_count):
       IsMappedUnusedHwTrigger(new_s, TrigIdxAt(new_s, i)) &&
       HwTriggerMatchesConfig(new_s, HwTriggerOf(new_s, TrigIdxAt(new_s, i)), TriggerConfigAt(new_s, i)))
  && (forall i in [0, trig_count):
       TrigState(new_s, TrigIdxAt(new_s, i)) == SavedPrivBits(new_s, TriggerConfigAt(new_s, i).trig_tdata1))
  && (forall i in [0, trig_count):
       HwTriggerOf(new_s, TrigIdxAt(new_s, i)).tdata1 == TriggerConfigAt(new_s, i).trig_tdata1 &&
       HwTriggerOf(new_s, TrigIdxAt(new_s, i)).tdata2 == TriggerConfigAt(new_s, i).trig_tdata2 &&
       HwTriggerOf(new_s, TrigIdxAt(new_s, i)).tdata3 == TriggerConfigAt(new_s, i).trig_tdata3)
  && (forall i in [0, trig_count):
       SharedMemWord(new_s, i * (XLEN / 2), 0) == TrigIdxAt(new_s, i))
  && (forall chain in TriggerConfigChains(new_s, 0, trig_count):
       TrigIdxValuesContiguous(new_s, chain) &&
       HwTriggersContiguous(new_s, chain))
}