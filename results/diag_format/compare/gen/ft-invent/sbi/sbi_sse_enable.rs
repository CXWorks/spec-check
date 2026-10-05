pub open spec fn sbi_sse_enable_spec(event_id: UInt32, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_STATE ==> EventAt(new_s, event_id).state == REGISTERED)
  && ((!(result == SBI_ERR_INVALID_STATE))
    ==> EventAt(new_s, event_id).state == ENABLED)
}