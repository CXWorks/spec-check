pub open spec fn sbi_debug_install_triggers_spec(trig_count: unsigned long, error: long, value: long, old_s: S, new_s: S) -> bool {
  (TriggerConfigsProcessedInIncreasingIndexOrderFromZero(new_s, trig_count))
  && (forall|i: UInt64| i < trig_count ==> !PreTrigIdxInUse(new_s, InstalledTrigIdx(i)) && !PreHwTriggerInUse(new_s, HwTriggerOf(InstalledTrigIdx(i)))))
  && (forall|i: UInt64| i < trig_count ==> HwTriggerMatchesConfig(new_s, HwTriggerOf(InstalledTrigIdx(i)), TrigConfigAt(new_s, i))))
  && (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vs == TrigConfigAt(new_s, i).tdata1.vs))
  && (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vu == TrigConfigAt(new_s, i).tdata1.vu))
  && (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).s == TrigConfigAt(new_s, i).tdata1.s))
  && (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).u == TrigConfigAt(new_s, i).tdata1.u))
  && (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata1 == TrigConfigAt(new_s, i).tdata1))
  && (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata2 == TrigConfigAt(new_s, i).tdata2))
  && (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata3 == TrigConfigAt(new_s, i).tdata3))
  && (forall|i: UInt64| i < trig_count ==> SharedMemWordLE(new_s, i * (XLEN / 2), 0) == InstalledTrigIdx(new_s, i))))
  && (forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> TrigIdxsContiguous(new_s, c)))
  && (forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> HwTriggersContiguous(new_s, c)))
  && ((error == 0) ==> (TriggerConfigsProcessedInIncreasingIndexOrderFromZero(new_s, trig_count)) &&
       (forall|i: UInt64| i < trig_count ==> !PreTrigIdxInUse(new_s, InstalledTrigIdx(i)) && !PreHwTriggerInUse(new_s, HwTriggerOf(InstalledTrigIdx(i))))) &&
       (forall|i: UInt64| i < trig_count ==> HwTriggerMatchesConfig(new_s, HwTriggerOf(InstalledTrigIdx(i)), TrigConfigAt(new_s, i)))) &&
       (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vs == TrigConfigAt(new_s, i).tdata1.vs)) &&
       (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vu == TrigConfigAt(new_s, i).tdata1.vu)) &&
       (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).s == TrigConfigAt(new_s, i).tdata1.s)) &&
       (forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).u == TrigConfigAt(new_s, i).tdata1.u)) &&
       (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata1 == TrigConfigAt(new_s, i).tdata1)) &&
       (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata2 == TrigConfigAt(new_s, i).tdata2)) &&
       (forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata3 == TrigConfigAt(new_s, i).tdata3)) &&
       (forall|i: UInt64| i < trig_count ==> SharedMemWordLE(new_s, i * (XLEN / 2), 0) == InstalledTrigIdx(new_s, i)))) &&
       (forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> TrigIdxsContiguous(new_s, c))) &&
       (forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> HwTriggersContiguous(new_s, c))))
  && ((error != 0)
    ==> TriggerConfigsProcessedInIncreasingIndexOrderFromZero(new_s, trig_count))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> !PreTrigIdxInUse(new_s, InstalledTrigIdx(i)) && !PreHwTriggerInUse(new_s, HwTriggerOf(InstalledTrigIdx(i)))))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> HwTriggerMatchesConfig(new_s, HwTriggerOf(InstalledTrigIdx(i)), TrigConfigAt(new_s, i))))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vs == TrigConfigAt(new_s, i).tdata1.vs))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).vu == TrigConfigAt(new_s, i).tdata1.vu))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).s == TrigConfigAt(new_s, i).tdata1.s))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> TrigState(new_s, InstalledTrigIdx(i)).u == TrigConfigAt(new_s, i).tdata1.u))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata1 == TrigConfigAt(new_s, i).tdata1))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata2 == TrigConfigAt(new_s, i).tdata2))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> HwTriggerOf(new_s, InstalledTrigIdx(i)).tdata3 == TrigConfigAt(new_s, i).tdata3))
  && ((error != 0)
    ==> forall|i: UInt64| i < trig_count ==> SharedMemWordLE(new_s, i * (XLEN / 2), 0) == InstalledTrigIdx(new_s, i))))
  && ((error != 0)
    ==> forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> TrigIdxsContiguous(new_s, c)))
  && ((error != 0)
    ==> forall|c: TriggerChain| IsTriggerChainInSharedMem(new_s, c, trig_count) ==> HwTriggersContiguous(new_s, c)))
  && (forall(s: S, i: UInt64)| i < trig_count ==> SharedMemWordLE(s, i * (XLEN / 2), 0) == InstalledTrigIdx(s, i))
    ==> SharedMemWordLE(new_s, i * (XLEN / 2), 0) == InstalledTrigIdx(new_s, i))
  )
}