pub open spec fn sbi_debug_install_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == 0)
    && (TriggerConfigsProcessedInIncreasingIndexOrder(0, trig_count(old_s)) ==> true)
    && (forall i in [0, trig_count(old_s)):
        IsMappedUnusedHwTrigger(TrigIdxAt(i, new_s))
        && HwTriggerMatchesConfig(HwTriggerOf(TrigIdxAt(i, new_s)), TriggerConfigAt(i, old_s))
    )
    && (forall i in [0, trig_count(old_s)):
        TrigState(TrigIdxAt(i, new_s)) == SavedPrivBits(TriggerConfigAt(i, old_s).trig_tdata1)
    )
    && (forall i in [0, trig_count(old_s)):
        HwTriggerOf(TrigIdxAt(i, new_s)).tdata1 == TriggerConfigAt(i, old_s).trig_tdata1
        && HwTriggerOf(TrigIdxAt(i, new_s)).tdata2 == TriggerConfigAt(i, old_s).trig_tdata2
        && HwTriggerOf(TrigIdxAt(i, new_s)).tdata3 == TriggerConfigAt(i, old_s).trig_tdata3
    )
    && (forall i in [0, trig_count(old_s)):
        SharedMemWord(i * (XLEN / 2), 0, new_s) == TrigIdxAt(i, new_s)
    )
    && (forall chain in TriggerConfigChains(0, trig_count(old_s)):
        TrigIdxValuesContiguous(chain, new_s)
        && HwTriggersContiguous(chain, new_s)
    )
}