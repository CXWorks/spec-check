pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && ((top as int) <= (base as int) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(base)
        && AddrIsGranuleAligned(top)
        && (top as int) > (base as int)
        && AddrRangeIsProtected(base, top, CurrentRealm(old_s)))
        ==> (result == RSI_SUCCESS
            && (out_top as int) > (base as int)
            && (out_top as int) <= (top as int)
            && RttEntriesInRangeRipas(old_s, RttAt(old_s, CurrentRealm(old_s).rtt_base), CurrentRealm(old_s).rtt_level_start as int, base, out_top, RipasToRmi(old_s, ripas))))
    && new_s == old_s
}