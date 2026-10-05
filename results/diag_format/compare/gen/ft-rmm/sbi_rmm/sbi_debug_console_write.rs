pub open spec fn sbi_debug_console_write_spec(num_bytes: UInt, base_addr_lo: Bits, base_addr_hi: Bits, error: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
  (!InputMemoryMeetsSection3_2(old_s, num_bytes, base_addr_lo, base_addr_hi) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!DebugConsoleWriteAllowed(old_s) ==> ResultEqual(error, SBI_ERR_DENIED))
  && (DebugConsoleIoError(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> uvalue == NumBytesWrittenToDebugConsole(new_s))
  && (ResultEqual(error, SBI_SUCCESS) ==> uvalue <= num_bytes)
  && ((InputMemoryMeetsSection3_2(old_s, num_bytes, base_addr_lo, base_addr_hi) &&
       DebugConsoleWriteAllowed(old_s) &&
       !DebugConsoleIoError(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
}