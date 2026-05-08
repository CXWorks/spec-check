pub open spec fn RSI_IPA_STATE_SET_spec(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    base: Address,
    top: Address,
    ripas: RsiRipas,
    flags: RsiRipasChangeFlags,
    result: RsiCommandReturnCode,
    new_base: Address,
    response: RsiResponse,
) -> bool {
    let base_align = AddrIsGranuleAligned(base);
    let top_align = AddrIsGranuleAligned(top);
    let size_valid = UInt(top) > UInt(base);
    let rgn_bound = AddrRangeIsProtected(base, top, realm);
    let ripas_valid = (ripas == RSI_EMPTY || ripas == RSI_RAM);

    (base_align && top_align && size_valid && rgn_bound && ripas_valid) ==> (RttEntriesInRangeRipas(
        s,
        RttAt(s, rec.rtt_addr),
        0,
        base,
        new_base,
        RipasDecode(s, ripas),
    ) && new_base == rec.ripas_addr && response == RecRipasResponseToRsi(s, rec) && result
        == RSI_SUCCESS) && (!base_align ==> result == RSI_ERROR_INPUT) && (!top_align && base_align
        ==> result == RSI_ERROR_INPUT) && (!size_valid && top_align && base_align ==> result
        == RSI_ERROR_INPUT) && (!rgn_bound && size_valid && top_align && base_align ==> result
        == RSI_ERROR_INPUT) && (!ripas_valid && rgn_bound && size_valid && top_align && base_align
        ==> result == RSI_ERROR_INPUT)
}