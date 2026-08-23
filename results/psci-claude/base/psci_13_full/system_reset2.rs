pub open spec fn system_reset2_spec(reset_type: UInt32, cookie: Bits64, result: Int32, old_s: S, new_s: S) -> bool {
    (((reset_type & 0x80000000u32) == 0) && (reset_type & 0x7FFFFFFFu32) != 0x0
        ==> result as int == PSCI_INVALID_PARAMETERS)
    && ((result as int == PSCI_INVALID_PARAMETERS || result as int == PSCI_NOT_SUPPORTED)
        ==> new_s == old_s)
}