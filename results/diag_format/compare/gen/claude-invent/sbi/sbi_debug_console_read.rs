pub open spec fn sbi_debug_console_read_spec(num_bytes: UInt64, base_addr_lo: UInt64, base_addr_hi: UInt64, error: i64, value: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidConsoleBuffer(old_s, num_bytes, base_addr_lo, base_addr_hi) ==> (error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((IsValidConsoleBuffer(old_s, num_bytes, base_addr_lo, base_addr_hi) && !ConsoleReadAllowed(old_s)) ==> (error == SBI_ERR_DENIED && new_s == old_s))
    && ((IsValidConsoleBuffer(old_s, num_bytes, base_addr_lo, base_addr_hi) && ConsoleReadAllowed(old_s) && ConsoleIoError(old_s)) ==> (error == SBI_ERR_FAILED && new_s == old_s))
    && ((IsValidConsoleBuffer(old_s, num_bytes, base_addr_lo, base_addr_hi) && ConsoleReadAllowed(old_s) && !ConsoleIoError(old_s)) ==> (
        error == SBI_SUCCESS
        && (value as int) <= (num_bytes as int)
        && ConsoleBytesWritten(old_s, new_s, base_addr_lo, base_addr_hi, value)
    ))
}
