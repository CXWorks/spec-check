pub open spec fn sbi_debug_console_write_spec(num_bytes: UInt64, base_addr_lo: UInt64, base_addr_hi: UInt64, result: sbiret, old_s: S, new_s: S) -> bool {
  (result.error == SBI_ERR_INVALID_PARAM ==> true)
  && (result.error == SBI_ERR_DENIED ==> true)
  && (result.error == SBI_ERR_FAILED ==> true)
  && ((!(result.error == SBI_ERR_INVALID_PARAM) &&
       result.error == SBI_SUCCESS)
    ==> true)
}