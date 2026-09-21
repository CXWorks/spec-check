pub open spec fn sbi_send_ipi_spec(hart_mask: UInt64, hart_mask_base: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> (true))
  && (result == SBI_ERR_FAILED ==> (true))
  && (result == SBI_SUCCESS ==> (true))
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       result == SBI_SUCCESS)
    ==> (true))
}