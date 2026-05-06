pub open spec fn rsi_mem_set_perm_index_spec(
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    new_cookie: u64,
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    perm_index: u64,
    cookie: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);

    // Failure condition: base_align
    (!AddrIsGranuleAligned(old_s, base) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: top_align
     && (!AddrIsGranuleAligned(old_s, top) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: size_valid
     && (UInt(top) <= UInt(base) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: rgn_bound
     && (!AddrRangeIsProtected(old_s, base, top, realm) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: perm_bound
     && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES(old_s) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: cookie
     && (!CookieIsValid(old_s, cookie, rec) ==> result
        == RSI_ERROR_INPUT)
    // Success conditions
     && ((AddrIsGranuleAligned(old_s, base) && AddrIsGranuleAligned(old_s, top) && UInt(top) > UInt(
        base,
    ) && AddrRangeIsProtected(old_s, base, top, realm) && perm_index < RMM_NUM_PERM_OVERLAY_INDICES(
        old_s,
    ) && CookieIsValid(old_s, cookie, rec)) ==> (result == RSI_SUCCESS
        && new_s.realms[realm].overlay_locked[perm_index as int] == MEM_PERM_LOCKED && new_base
        == rec.s2ap_addr && response == RecS2APResponseToRsi(old_s, rec) && CookieIsGenerated(
        old_s,
        new_s,
        new_cookie,
    )))
}