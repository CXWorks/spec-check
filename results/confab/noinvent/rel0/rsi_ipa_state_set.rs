pub open spec fn rsi_ipa_state_set_spec(result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, base: Address, top: Address, ripas: RsiRipas, flags: RsiRipasChangeFlags, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && (((top as int) <= (base as int)) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (((ripas != RSI_EMPTY) && (ripas != RSI_RAM)) ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, base)
        && AddrIsGranuleAligned(old_s, top)
        && ((top as int) > (base as int))
        && AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s))
        && ((ripas == RSI_EMPTY) || (ripas == RSI_RAM)))
        ==> (new_base == CurrentRec(new_s).ripas_addr
            && response == RecRipasChangeResponse(new_s, CurrentRec(new_s))))
}