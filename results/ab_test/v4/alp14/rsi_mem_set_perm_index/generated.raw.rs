pub open spec fn RSI_MEM_SET_PERM_INDEX_spec(
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    perm_index: u64,
    cookie: Bits64,
    realm: RmmRealm,
    rec: RmmRec,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    new_cookie: Bits64
) -> bool {
    ((!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT) &&
    ((UInt(top) as int) <= (UInt(base) as int) ==> result == RSI_ERROR_INPUT) &&
    (!AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) &&
    ((perm_index as int) >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT) &&
    (!IsValidCookie(old_s, cookie) ==> result == RSI_ERROR_INPUT) &&
    ((AddrIsGranuleAligned(base) &&
      AddrIsGranuleAligned(top) &&
      (UInt(top) as int) > (UInt(base) as int) &&
      AddrRangeIsProtected(old_s, base, top, realm) &&
      ((perm_index as int) < RMM_NUM_PERM_OVERLAY_INDICES) &&
      IsValidCookie(old_s, cookie)) ==>
        (realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED &&
         new_base == rec.s2ap_addr &&
         response == RecS2APResponseToRsi(old_s, rec) &&
         IsNewCookieGenerated(old_s, new_s, new_cookie))))
}