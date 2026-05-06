```verus
pub open spec fn RSI_MEM_SET_PERM_INDEX_spec(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    base: Address,
    top: Address,
    perm_index: u64,
    cookie: u64,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    new_cookie: u64
) -> bool {
    let rmi_status_code = RsiCommandReturnCodeToRmiStatusCode(result);
    (
        // Failure conditions
        (!AddrIsGranuleAligned(base) ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        (!AddrIsGranuleAligned(top) ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        (UInt(top) <= UInt(base) ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        (!AddrRangeIsProtected(s, base, top, realm) ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        (!CookieIsValid(s, cookie) ==> rmi_status_code == RmiStatusCode::RSI_ERROR_INPUT) &&
        // Success conditions
        (rmi_status_code == RmiStatusCode::RSI_SUCCESS ==> 
            realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED &&
            new_base == rec.s2ap_addr &&
            response == RecS2APResponseToRsi(s, rec) &&
            CookieIsValid(s, new_cookie)
        )
    )
}
```