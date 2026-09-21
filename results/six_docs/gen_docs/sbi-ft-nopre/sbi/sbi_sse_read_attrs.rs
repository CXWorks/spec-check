pub open spec fn sbi_sse_read_attrs_spec(event_id: uint32_t, base_attr_id: uint32_t, attr_count: uint32_t, output_phys_lo: unsigned long, output_phys_hi: unsigned long, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> (event_id == 0 || attr_count == 0))
  && (result == SBI_ERR_INVALID_ADDRESS ==> output_phys_lo % (XLEN / 8) == 0)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       !(result == SBI_ERR_INVALID_ADDRESS))
    ==> result == SBI_SUCCESS)
}