```verus
pub open spec fn rsi_ipa_state_set_spec(
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    ripas: RsiRipas,
    flags: RsiRipasChangeFlags,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    
    // Failure conditions
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT) &&
    (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT) &&
    (!AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) &&
    ((ripas != RSI_EMPTY && ripas != RSI_RAM) ==> result == RSI_ERROR_INPUT) &&
    
    // Success conditions
    (AddrIsGranuleAligned(old_s, base) &&
     AddrIsGranuleAligned(old_s, top) &&
     UInt(top) > UInt(base) &&
     AddrRangeIsProtected(old_s, base, top, realm) &&
     (ripas == RSI_EMPTY || ripas == RSI_RAM) ==>
        (RipasInRangeEqual(new_s, base, new_base, ripas) &&
         new_base == rec.ripas_addr &&
         response == RecRipasResponseToRsi(old_s, rec)))
}
```