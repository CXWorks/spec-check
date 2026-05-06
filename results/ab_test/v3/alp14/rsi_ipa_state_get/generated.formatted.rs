pub open spec fn RSI_IPA_STATE_GET_spec(
    old_s: S,
    base: Address,
    top: Address,
    realm: RmmRealm,
    result: RsiCommandReturnCode,
    out_top: Address,
    ripas: RsiRipas,
) -> bool {
    // Failure conditions
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT) &&
    (UInt64(top) <= UInt64(base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) &&
    
    // Success conditions
    (result == RSI_SUCCESS ==>
        (out_top > base &&
         out_top <= top &&
         RttEntriesInRangeRipas(old_s, RttAt(old_s, base), 0, base, out_top, RipasToRmm(old_s, ripas)) &&
         (forall addr: Address :: base <= addr && addr < out_top ==>
            RttGetRipas(old_s, realm, addr) == ripas)))
}