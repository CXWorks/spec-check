pub open spec fn sbi_sse_write_attrs_spec(event_id: uint32_t, base_attr_id: uint32_t, attr_count: uint32_t, input_phys_lo: unsigned long, input_phys_hi: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_ADDRESS ==> (input_phys_lo % (64 / 8) == 0))
  && ((!(result == SBI_ERR_INVALID_ADDRESS))
    ==> result == SBI_SUCCESS)
}