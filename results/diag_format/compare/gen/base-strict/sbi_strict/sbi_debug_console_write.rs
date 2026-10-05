pub open spec fn sbi_debug_console_write_spec(result: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
    (!IsValidSharedMemory(old_s, num_bytes(old_s), base_addr_lo(old_s), base_addr_hi(old_s)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!DebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (DebugConsoleIoError(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (uvalue <= num_bytes(old_s) && DebugConsoleWritten(PhysAddr(base_addr_hi(old_s), base_addr_lo(old_s)), uvalue)))
}