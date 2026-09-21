pub open spec fn sbi_sse_register_spec(event_id: uint32_t, handler_entry_pc: unsigned long, handler_entry_arg: unsigned long, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_STATE ==> true)
  && (result == SBI_ERR_INVALID_PARAM ==> true)
  && ((!(result == SBI_ERR_NOT_SUPPORTED) &&
       result == SBI_SUCCESS)
    ==> true)
  && ((!(result == SBI_ERR_NOT_SUPPORTED) &&
       result == SBI_ERR_INVALID_STATE)
    ==> true)
  && ((!(result == SBI_ERR_NOT_SUPPORTED) &&
       result == SBI_ERR_INVALID_PARAM)
    ==> true)
  && (result == SBI_ERR_NOT_SUPPORTED ==> true)
  && (result == SBI_ERR_INVALID_PARAM ==> true)
  && ((result == SBI_SUCCESS)
    ==> true)
}