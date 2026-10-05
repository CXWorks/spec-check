pub open spec fn sbi_debug_console_write_byte_spec(byte: u8, result: SbiRet, old_s: S, new_s: S) -> bool {
    (result.uvalue == 0)
    && (result.error == SBI_SUCCESS || result.error == SBI_ERR_DENIED || result.error == SBI_ERR_FAILED)
    && (!DebugConsoleWriteAllowed(old_s) ==> (result.error == SBI_ERR_DENIED && new_s == old_s))
    && ((DebugConsoleWriteAllowed(old_s) && DebugConsoleIoError(old_s, byte)) ==> result.error == SBI_ERR_FAILED)
    && ((DebugConsoleWriteAllowed(old_s) && !DebugConsoleIoError(old_s, byte)) ==> (result.error == SBI_SUCCESS && DebugConsoleByteWritten(old_s, new_s, byte)))
}
