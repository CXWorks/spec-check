pub open spec fn sbi_debug_console_write_byte_spec(result: SbiError, uvalue: u64, byte: u8, old_s: S, new_s: S) -> bool {
    (!DebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (DebugConsoleIoError(byte) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (uvalue == 0 && DebugConsoleByteWritten(byte)))
    && (ResultEqual(result, SBI_SUCCESS) ==> DebugConsole.output == old_s.DebugConsole.output)
}