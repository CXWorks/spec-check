pub open spec fn system_reset2_spec(result: int, reset_type: UInt32, cookie: Bits64, old_s: S, new_s: S) -> bool {
    ((reset_type as int) < 0x80000000 && (reset_type as int) != 0x0 ==> result == PSCI_INVALID_PARAMETERS)
    && ((reset_type as int) < 0x80000000 && (reset_type as int) != 0x0 ==> new_s == old_s)
    && (result == PSCI_NOT_SUPPORTED ==> new_s == old_s)
}