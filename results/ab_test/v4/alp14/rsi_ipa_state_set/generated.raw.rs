pub open spec fn RSI_IPA_STATE_SET_spec(old_s: S, new_s: S, base: Address, top: Address, ripas: RsiRipas, flags: RsiRipasChangeFlags, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, realm: RmmRealm, rec: RmmRec) -> bool {
    let base_align_fail = !AddrIsGranuleAligned(base);
    let top_align_fail = !AddrIsGranuleAligned(top);
    let size_valid_fail = (top as int) <= (base as int);
    let rgn_bound_fail = !AddrRangeIsProtected(base, top, realm);
    let ripas_valid_fail = (ripas != RSI_EMPTY) && (ripas != RSI_RAM);
    
    (base_align_fail ==> result == RSI_ERROR_INPUT) &&
    (top_align_fail ==> result == RSI_ERROR_INPUT) &&
    (size_valid_fail ==> result == RSI_ERROR_INPUT) &&
    (rgn_bound_fail ==> result == RSI_ERROR_INPUT) &&
    (ripas_valid_fail ==> result == RSI_ERROR_INPUT) &&
    
    (!base_align_fail && !top_align_fail && !size_valid_fail && !rgn_bound_fail && !ripas_valid_fail ==>
        (result == RSI_SUCCESS &&
         new_base == rec.ripas_addr &&
         response == RecRipasResponseToRsi(old_s, rec)))
}