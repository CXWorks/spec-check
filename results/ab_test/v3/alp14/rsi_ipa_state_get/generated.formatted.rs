pub open spec fn rsi_ipa_state_get_spec(result: RsiCommandReturnCode, base: Address, top: Address, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
    let realm = old_s;
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && (((top as int) <= (base as int)) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, realm) ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && ((top as int) > (base as int)) && AddrRangeIsProtected(base, top, realm))
        ==> (result.is_Ok() && out_top > base && out_top <= top))
}