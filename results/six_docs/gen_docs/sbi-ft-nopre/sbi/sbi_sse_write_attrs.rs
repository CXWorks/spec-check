pub open spec fn sbi_sse_write_attrs_spec(event_id: uint32_t, base_attr_id: uint32_t, attr_count: uint32_t, input_phys_lo: unsigned long, input_phys_hi: unsigned long, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((input_phys_lo % (XLEN / 8) == 0) &&
   result == SBI_SUCCESS)
  && (result == SBI_ERR_INVALID_ADDRESS &&
       !(input_phys_lo % (XLEN / 8) == 0))
  && ((result == SBI_SUCCESS ||
       result == SBI_ERR_NOT_SUPPORTED ||
       result == SBI_ERR_INVALID_PARAM ||
       result == SBI_ERR_DENIED ||
       result == SBI_ERR_INVALID_STATE ||
       result == SBI_ERR_BAD_RANGE ||
       result == SBI_ERR_INVALID_ADDRESS ||
       result == SBI_ERR_FAILED)
    ==> (GlobalEventAt(new_s, event_id).attributes[base_attr_id + 0 as int] == GlobalEventAt(old_s, event_id).attributes[base_attr_id + 0 as int]))
  && (result == SBI_SUCCESS && attr_count > 0
    ==> (GlobalEventAt(new_s, event_id).attributes[base_attr_id + 1 as int] == GlobalEventAt(old_s, event_id).attributes[base_attr_id + 1 as int]))
}