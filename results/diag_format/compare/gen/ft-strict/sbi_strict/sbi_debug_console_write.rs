pub open spec fn sbi_debug_console_write_spec(num_bytes: UInt, base_addr_lo: Address, base_addr_hi: Address, result: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
  (!IsValidSharedMemory(old_s, num_bytes, base_addr_lo, base_addr_hi) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!DebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (DebugConsoleIoError(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> uvalue <= num_bytes)
  && (result == SBI_SUCCESS ==> DebugConsoleWritten(new_s, PhysAddr(base_addr_hi, base_addr_lo), uvalue))
  && ((IsValidSharedMemory(old_s, num_bytes, base_addr_lo, base_addr_hi) &&
       DebugConsoleWriteAllowed(old_s) &&
       !DebugConsoleIoError(old_s))
    ==> result == SBI_SUCCESS)
}