pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let base_aligned = AddrIsGranuleAligned(old_s, base);
    let top_aligned = AddrIsGranuleAligned(old_s, top);
    let top_greater_base = top > base;
    let range_protected = AddrRangeIsProtected(old_s, base, top, realm);
    let perm_index_valid = perm_index < RMM_NUM_PERM_OVERLAY_INDICES;
    let cookie_valid = IsValidCookie(old_s, realm, cookie);
    let perm_index_locked = realm.overlay_locked[perm_index as usize] == MEM_PERM_LOCKED;
    let new_base_equal_s2ap = new_base == rec.s2ap_addr;
    let response_equal_rec = response == RecS2APResponseToRsi(old_s, rec);
    let new_cookie_generated = new_cookie != cookie;
    (
        (!base_aligned ==> result == RSI_ERROR_INPUT)
        && (!top_aligned ==> result == RSI_ERROR_INPUT)
        && (!top_greater_base ==> result == RSI_ERROR_INPUT)
        && (!range_protected ==> result == RSI_ERROR_INPUT)
        && (!perm_index_valid ==> result == RSI_ERROR_INPUT)
        && (!cookie_valid ==> result == RSI_ERROR_INPUT)
        && (result == RSI_SUCCESS ==> (perm_index_locked && new_base_equal_s2ap && response_equal_rec && new_cookie_generated))
        && (result == RSI_SUCCESS ==> new_base == base)
        && (result == RSI_SUCCESS ==> new_cookie != cookie)
        && (result == RSI_SUCCESS ==> new_s.overlay_locked[perm_index as usize] == MEM_PERM_LOCKED)
        && (result == RSI_SUCCESS ==> new_s == old_s)
    )
}