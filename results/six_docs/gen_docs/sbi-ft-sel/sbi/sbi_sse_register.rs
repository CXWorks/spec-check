pub open spec fn sbi_sse_register_spec(event_id: UInt32, handler_entry_pc: UInt64, handler_entry_arg: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
  ((handler_entry_pc % 2) != 0 ==> result.code == SBI_ERR_INVALID_PARAM)
  && (result.code == SBI_ERR_INVALID_STATE ==> true)
  && ((!(result.code == SBI_ERR_INVALID_STATE) &&
       result.code != SBI_ERR_NOT_SUPPORTED &&
       result.code != SBI_ERR_INVALID_PARAM)
    ==> result.code == SBI_SUCCESS)
  && ((!(result.code == SBI_ERR_INVALID_STATE) &&
       result.code != SBI_ERR_NOT_SUPPORTED &&
       result.code != SBI_ERR_INVALID_PARAM)
    ==> result.code == SBI_SUCCESS)
}