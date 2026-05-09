pub open spec fn rsi_ipa_state_set_spec(result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, old_s: S, new_s: S, base: Address, top: Address, ripas: RsiRipas, flags: RsiRipasChangeFlags) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, realm) ==> result == RSI_ERROR_INPUT)
    && (((ripas != RSI_EMPTY) && (ripas != RSI_RAM)) ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && UInt(top) > UInt(base) && AddrRangeIsProtected(base, top, realm) && ((ripas == RSI_EMPTY) || (ripas == RSI_RAM)))
        ==> (result == RSI_SUCCESS && RipasInRangeEqual(new_s, realm, base, new_base, ripas) && new_base == rec.ripas_addr && response == RecRipasResponseToRsi(new_s, rec)))
}