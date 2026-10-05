pub open spec fn sbi_debug_enable_triggers_spec(trig_idx_base: unsigned long, trig_idx_mask: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : trig_idx >= trig_max) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(trig_idx)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : MappedHwTrigger(trig_idx).vs == trig_state(trig_idx).saved.vs))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : MappedHwTrigger(trig_idx).vu == trig_state(trig_idx).saved.vu))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : MappedHwTrigger(trig_idx).s == trig_state(trig_idx).saved.s))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : MappedHwTrigger(trig_idx).u == trig_state(trig_idx).saved.u))
  && ((!Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : trig_idx >= trig_max) &&
       !Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(trig_idx)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(trig_idx_base).vs == MappedHwTrigger(trig_idx_base).vs)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(trig_idx_base).vu == MappedHwTrigger(trig_idx_base).vu)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(trig_idx_base).s == MappedHwTrigger(trig_idx_base).s)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(trig_idx_base).u == MappedHwTrigger(trig_idx_base).u)
}