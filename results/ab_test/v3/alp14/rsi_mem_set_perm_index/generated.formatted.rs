pub open spec fn RSI_MEM_SET_PERM_INDEX_spec(
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    perm_index: u64,
    cookie: u64,
    realm: RmmRealm,
    rec: RmmRec,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    new_cookie: u64,
) -> bool {
    // Failure condition: base not granule aligned
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: top not granule aligned
    (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: invalid size
    (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: region not protected
    (!AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: invalid permission index
    (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES(old_s) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: invalid cookie
    (!CookieIsValid(old_s, cookie) ==> result == RSI_ERROR_INPUT)
        &&
    // Success conditions (when no failure conditions triggered)
    ((AddrIsGranuleAligned(old_s, base) && AddrIsGranuleAligned(old_s, top) && UInt(top) > UInt(
        base,
    ) && AddrRangeIsProtected(old_s, base, top, realm) && perm_index < RMM_NUM_PERM_OVERLAY_INDICES(
        old_s,
    ) && CookieIsValid(old_s, cookie)) ==> (result == RSI_SUCCESS
        && new_s.realms[realm].overlay_locked[perm_index as int] == MEM_PERM_LOCKED && new_base
        == rec.s2ap_addr && response == RecS2APResponseToRsi(old_s, rec) && new_cookie != cookie))
}