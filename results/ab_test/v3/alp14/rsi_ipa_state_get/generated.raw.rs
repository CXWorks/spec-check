pub open spec fn rsi_ipa_state_get_spec(result: RsiCommandReturnCode, base: Address, top: Address, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, CurrentRealm()) ==> result == RSI_ERROR_INPUT)
    && (
        (AddrIsGranuleAligned(base)
        && AddrIsGranuleAligned(top)
        && UInt(top) > UInt(base)
        && AddrRangeIsProtected(base, top, CurrentRealm()))
        ==>
        (result == RSI_SUCCESS
        && out_top > base
        && out_top <= top
        && new_s == old_s)
    )
}