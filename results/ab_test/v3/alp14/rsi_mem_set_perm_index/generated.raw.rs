pub open spec fn rsi_mem_set_perm_index_spec(result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, base: Address, top: Address, perm_index: u64, cookie: Bits64, old_s: S, new_s: S) -> bool {
    let realm = old_s;
    let rec = old_s;
    (
        (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
        && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
        && ((top as int) <= (base as int) ==> result == RSI_ERROR_INPUT)
        && (!AddrRangeIsProtected(base, top, realm) ==> result == RSI_ERROR_INPUT)
        && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
        && (!CookieIsValid(cookie, old_s) ==> result == RSI_ERROR_INPUT)
        && (
            (AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && ((top as int) > (base as int)) && AddrRangeIsProtected(base, top, realm) && perm_index < RMM_NUM_PERM_OVERLAY_INDICES && CookieIsValid(cookie, old_s))
            ==> (result.is_Ok() && new_base == rec.s2ap_addr && response == RecS2APResponseToRsi(old_s, rec) && new_s.overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
        )
    )
}