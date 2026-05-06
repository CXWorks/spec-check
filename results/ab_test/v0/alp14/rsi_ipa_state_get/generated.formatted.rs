pub open spec fn RSI_IPA_STATE_GET_spec(
    s: S,
    realm: RmmRealm,
    base: Address,
    top: Address,
    out_top: Address,
    ripas: RsiRipas,
) -> bool {
    let rtt = RttAt(s, realm.s_rtt_addr);
    (
    // Failure conditions
    (!AddrIsGranuleAligned(s, base) ==> false) && (!AddrIsGranuleAligned(s, top) ==> false) && (
    UInt64(top) <= UInt64(base) ==> false) && (!AddrRangeIsProtected(s, base, top, realm) ==> false)
        &&
    // Success conditions
    (AddrIsGranuleAligned(s, base) && AddrIsGranuleAligned(s, top) && UInt64(top) > UInt64(base)
        && AddrRangeIsProtected(s, base, top, realm) ==> (out_top > base && out_top <= top
        && RttEntriesInRangeRipas(
        s,
        rtt,
        realm.s_rtt_level_start,
        base,
        out_top,
        RipasToRmm(s, ripas),
    ) && (out_top == top || RttEntryAt(
        s,
        rtt,
        RttEntryIndex(s, out_top, realm.s_rtt_level_start),
    ).entry_ripas != RipasToRmm(s, ripas)))))
}