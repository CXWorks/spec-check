pub open spec fn sbi_hart_start_spec(result: SbiRet, hartid: UInt64, start_addr: UInt64, opaque: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidHartId(old_s, hartid) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((IsValidHartId(old_s, hartid) && !IsValidStartAddr(old_s, start_addr)) ==> (result.error == SBI_ERR_INVALID_ADDRESS && new_s == old_s))
    && ((IsValidHartId(old_s, hartid) && IsValidStartAddr(old_s, start_addr) && HartStateOf(old_s, hartid) != SBI_HSM_STATE_STOPPED) ==> (result.error == SBI_ERR_ALREADY_AVAILABLE && new_s == old_s))
    && ((IsValidHartId(old_s, hartid) && IsValidStartAddr(old_s, start_addr) && HartStateOf(old_s, hartid) == SBI_HSM_STATE_STOPPED) ==> (
        (result.error == SBI_SUCCESS
            && (HartStateOf(new_s, hartid) == SBI_HSM_STATE_START_PENDING || HartStateOf(new_s, hartid) == SBI_HSM_STATE_STARTED)
            && HartStartAddr(new_s, hartid) == start_addr
            && HartStartOpaque(new_s, hartid) == opaque
            && HartStartMode(new_s, hartid) == PRIV_MODE_SUPERVISOR
            && (forall|h: UInt64| h != hartid ==> HartStateOf(new_s, h) == HartStateOf(old_s, h)))
        || (result.error == SBI_ERR_FAILED && new_s == old_s)
    ))
}
