pub open spec fn sbi_debug_console_write_spec(error: SbiErrorCode, uvalue: UInt, num_bytes: UInt, base_addr_lo: Bits, base_addr_hi: Bits, old_s: S, new_s: S) -> bool {
    (!InputMemoryMeetsSection3_2(num_bytes, base_addr_lo, base_addr_hi) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!DebugConsoleWriteAllowed() ==> ResultEqual(error, SBI_ERR_DENIED))
    && (DebugConsoleIoError() ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> uvalue == NumBytesWrittenToDebugConsole())
    && (ResultEqual(error, SBI_SUCCESS) ==> uvalue <= num_bytes)
    && (ResultEqual(error, SBI_SUCCESS) ==> new_s.DebugConsole.output == old_s.DebugConsole.output)
}