pub open spec fn rsi_ipa_state_get_spec(
    result: RsiCommandReturnCode,
    out_top: Address,
    ripas: RsiRipas,
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
) -> bool {
    let realm = CurrentRealm(old_s);

    // Failure condition: base_align
    ((!AddrIsGranuleAligned(old_s, base)) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: end_align
     && ((!AddrIsGranuleAligned(old_s, top)) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: size_valid
     && ((UInt(old_s, top) <= UInt(old_s, base)) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: rgn_bound
     && ((!AddrRangeIsProtected(old_s, base, top, realm)) ==> result
        == RSI_ERROR_INPUT)
    // Success condition: ripas
     && ((result == RSI_SUCCESS) ==> (out_top > base && out_top <= top && RttEntriesInRangeRipas(
        old_s,
        RttAt(old_s, realm.rtt_base),
        RttLevelIsStarting(old_s, realm, 0),
        base,
        out_top,
        RipasDecode(old_s, ripas),
    ) && new_s == old_s))
}