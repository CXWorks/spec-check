pub open spec fn rsi_mem_set_perm_index_spec(
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    new_cookie: Bits64,
    base: Address,
    top: Address,
    perm_index: u64,
    cookie: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(top)
        ==> result == RSI_ERROR_INPUT) && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT)
        && (!AddrRangeIsProtected(base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
        && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT) && (
    !CookieIsValid(old_s, cookie) ==> result == RSI_ERROR_INPUT) && ((AddrIsGranuleAligned(base)
        && AddrIsGranuleAligned(top) && UInt(top) > UInt(base) && AddrRangeIsProtected(
        base,
        top,
        CurrentRealm(old_s),
    ) && perm_index < RMM_NUM_PERM_OVERLAY_INDICES && CookieIsValid(old_s, cookie)) ==> (result
        == RSI_SUCCESS && new_base == CurrentRec(old_s).s2ap_addr && response
        == RecS2APResponseToRsi(old_s, CurrentRec(old_s)) && NewRealmState(
        old_s,
        new_s,
        perm_index,
    )))
}