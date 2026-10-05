pub open spec fn sbi_debug_console_write_byte_spec(byte: uint8_t, result: SBI error code, uvalue: unsigned long, old_s: S, new_s: S) -> bool {
  (!DebugConsoleWriteAllowed(old_s) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (DebugConsoleIoError(old_s, byte) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> uvalue == 0)
  && (result == SBI_SUCCESS ==> DebugConsoleByteWritten(new_s, byte))
  && ((DebugConsoleWriteAllowed(old_s) &&
       !DebugConsoleIoError(old_s, byte))
    ==> result == SBI_SUCCESS)
}