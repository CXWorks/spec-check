pub open spec fn sbi_sse_register_spec(event_id: uint32_t, handler_entry_pc: unsigned long, handler_entry_arg: unsigned long, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((handler_entry_pc % 2 != 0) ==> result == SBI_ERR_INVALID_PARAM)
  && (result == SBI_ERR_INVALID_STATE ==> EventAt(new_s, event_id).state == REGISTERED)
  && (result == SBI_SUCCESS ==> EventAt(new_s, event_id).state == REGISTERED)
  && ((!(handler_entry_pc % 2 != 0) &&
       result == SBI_SUCCESS)
    ==> EventAt(new_s, event_id).state == REGISTERED)
  && (result != SBI_SUCCESS
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
  && (result == SBI_ERR_NOT_SUPPORTED
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
  && (result == SBI_ERR_INVALID_PARAM
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
  && (result == SBI_ERR_INVALID_STATE
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
}