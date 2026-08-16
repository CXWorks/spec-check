pub open spec fn rsi_host_call_spec(
    fid: UInt64,
    addr: Address,
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsAligned(old_s, addr, 256) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        addr,
        CurrentRealm(old_s),
    ) ==> result == RSI_ERROR_INPUT) && (AddrIsAligned(old_s, addr, 256) && AddrIsProtected(
        old_s,
        addr,
        CurrentRealm(old_s),
    ) ==> result == RSI_SUCCESS)
}