pub open spec fn RSI_IPA_STATE_SET_spec(
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    ripas: RsiRipas,
    flags: RsiRipasChangeFlags,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);

    // Failure conditions (pre-conditions imply error result)
    ((!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(
        old_s,
        top,
    ) ==> result == RSI_ERROR_INPUT) && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT) && (
    !AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) && ((ripas
        != RSI_EMPTY && ripas != RSI_RAM) ==> result == RSI_ERROR_INPUT))
        &&
    // Success conditions (when no failure conditions met)
    ((AddrIsGranuleAligned(old_s, base) && AddrIsGranuleAligned(old_s, top) && UInt(top) > UInt(
        base,
    ) && AddrRangeIsProtected(old_s, base, top, realm) && (ripas == RSI_EMPTY || ripas == RSI_RAM))
        ==> (result == RSI_SUCCESS && RttEntriesInRangeRipas(
        new_s,
        RttAt(new_s, base),
        3,
        base,
        new_base,
        ripas,
    ) && new_base == rec.ripas_addr && response == RecRipasResponseToRsi(new_s, rec)))
}