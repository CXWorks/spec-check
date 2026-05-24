pub open spec fn rsi_ipa_state_set_spec(result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, old_s: S, new_s: S, base: Address, top: Address, ripas: RsiRipas, flags: RsiRipasChangeFlags) -> bool {
    let realm = CurrentRealm();
    let rec = CurrentRec();
    (!AddrIsGranuleAligned(base) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (((top as int) <= (base as int)) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, realm) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (((ripas != RSI_EMPTY) && (ripas != RSI_RAM)) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && ((top as int) > (base as int)) && AddrRangeIsProtected(base, top, realm) && ((ripas == RSI_EMPTY) || (ripas == RSI_RAM)))
        ==> (result.is_Ok() && RttEntriesInRangeRipas(new_s, RttAt(new_s, RttBase(realm)), RttStartLevel(realm), base, new_base, RipasDecode(ripas)) && new_base == rec.ripas_addr && response == RecRipasResponseToRsi(new_s, rec)))
}