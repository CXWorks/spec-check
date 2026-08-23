pub open spec fn system_off2_spec(result: int, hibernate_type: UInt32, cookie: UInt64, old_s: S, new_s: S) -> bool {
    ((hibernate_type as int) != 0x0 && (hibernate_type as int) != 0x1 ==> result == PSCI_INVALID_PARAMETERS)
    && ((cookie as int) != 0 ==> result == PSCI_INVALID_PARAMETERS)
    && (result == PSCI_INVALID_PARAMETERS || result == PSCI_NOT_SUPPORTED ==> new_s == old_s)
}