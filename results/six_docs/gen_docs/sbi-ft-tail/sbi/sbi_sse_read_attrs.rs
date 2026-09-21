pub open spec fn sbi_sse_read_attrs_spec(event_id: uint32_t, base_attr_id: uint32_t, attr_count: uint32_t, output_phys_lo: unsigned long, output_phys_hi: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> true)
  && (result == SBI_ERR_BAD_RANGE ==> true)
  && (result == SBI_ERR_INVALID_ADDRESS ==> true)
  && (result == SBI_ERR_FAILED ==> true)
  && (result == SBI_SUCCESS ==> true)
  && (result == SBI_ERR_NOT_SUPPORTED ==> true)
}