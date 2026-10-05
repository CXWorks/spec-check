pub open spec fn sbi_debug_console_write_byte_spec(result: SbiErrorCode, uvalue: UInt64, old_s: S, new_s: S) -> bool {
    (!IsDebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (DebugConsoleIoError(old_s, byte) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> ByteWrittenToDebugConsole(byte))
    && (ResultEqual(result, SBI_SUCCESS) ==> uvalue == 0)
}