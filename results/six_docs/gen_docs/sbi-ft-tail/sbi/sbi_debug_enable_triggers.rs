pub open spec fn sbi_debug_enable_triggers_spec(trig_idx_base: UInt64, trig_idx_mask: UInt64, result: SBI_SBI_RETURN_CODE, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> true)
  && (result == SBI_SUCCESS ==> true)
}