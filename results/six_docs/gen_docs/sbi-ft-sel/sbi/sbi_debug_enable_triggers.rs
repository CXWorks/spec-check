pub open spec fn sbi_debug_enable_triggers_spec(trig_idx_base: UInt64, trig_idx_mask: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result.error == SBI_ERR_INVALID_PARAM ==> true)
  && (result.error == SBI_SUCCESS ==> true)
}