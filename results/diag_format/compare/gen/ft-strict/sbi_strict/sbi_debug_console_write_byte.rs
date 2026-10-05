pub open spec fn sbi_debug_console_write_byte_spec(byte: UInt8, result: SbiErrorCode, uvalue: UInt64, old_s: S, new_s: S) -> bool {
  (!IsDebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (DebugConsoleIoError(old_s, byte) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ByteWrittenToDebugConsole(new_s, byte))
  && (result == SBI_SUCCESS ==> uvalue == 0)
  && ((IsDebugConsoleWriteAllowed(old_s) &&
       !DebugConsoleIoError(old_s, byte))
    ==> result == SBI_SUCCESS)
}