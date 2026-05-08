pub open spec fn RSI_IPA_STATE_GET_spec(s: S, realm: RmmRealm, base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas) -> bool {
    ((!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT) &&
    ((UInt(top) <= UInt(base)) ==> result == RSI_ERROR_INPUT) &&
    ((!AddrRangeIsProtected(s, base, top, realm)) ==> result == RSI_ERROR_INPUT) &&
    ((result == RSI_SUCCESS) ==> (
        UInt(out_top) > UInt(base) &&
        UInt(out_top) <= UInt(top) &&
        RttEntriesInRangeRipas(s, RttAt(s, base), RttLevelSize(s, 0), base, out_top, RipasToRmi(s, ripas))
    )))
}