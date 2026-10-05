pub open spec fn sbi_debug_console_write_spec(num_bytes: UInt64, base_addr_lo: UInt64, base_addr_hi: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!SbiSharedMemoryValid(old_s, num_bytes, base_addr_lo, base_addr_hi)
        ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((SbiSharedMemoryValid(old_s, num_bytes, base_addr_lo, base_addr_hi)
        && !DebugConsoleWriteAllowed(old_s))
        ==> (result.error == SBI_ERR_DENIED && new_s == old_s))
    && ((SbiSharedMemoryValid(old_s, num_bytes, base_addr_lo, base_addr_hi)
        && DebugConsoleWriteAllowed(old_s)
        && DebugConsoleIoError(old_s))
        ==> (result.error == SBI_ERR_FAILED && new_s == old_s))
    && ((SbiSharedMemoryValid(old_s, num_bytes, base_addr_lo, base_addr_hi)
        && DebugConsoleWriteAllowed(old_s)
        && !DebugConsoleIoError(old_s))
        ==> (result.error == SBI_SUCCESS
            && (result.uvalue as int) <= (num_bytes as int)
            && DebugConsoleBytesWritten(old_s, new_s, base_addr_lo, base_addr_hi, result.uvalue as int)))
}
