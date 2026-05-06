pub open spec fn rsi_ipa_state_set_spec(
    base: Address,
    top: Address,
    ripas: RsiRipas,
    flags: RsiRipasChangeFlags,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(
        old_s,
        top,
    ) ==> result == RSI_ERROR_INPUT) && ((top) <= (base) ==> result == RSI_ERROR_INPUT) && (
    !AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT) && (
    (ripas != RSI_EMPTY) && (ripas != RSI_RAM) ==> result == RSI_ERROR_INPUT) && (result
        == RSI_SUCCESS ==> new_base == CurrentRec(new_s).ripas_addr) && (result == RSI_SUCCESS
        ==> response == RecRipasResponseToRsi(new_s, CurrentRec(new_s))) && ((AddrIsGranuleAligned(
        old_s,
        base,
    ) && AddrIsGranuleAligned(old_s, top) && !((top) <= (base)) && AddrRangeIsProtected(
        old_s,
        base,
        top,
        CurrentRealm(old_s),
    ) && !((ripas != RSI_EMPTY) && (ripas != RSI_RAM))) ==> result == RSI_SUCCESS)
}