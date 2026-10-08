pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    // Failure: src not aligned to granule size
    (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: src not accessible from NS
    (!PaIsDelegableNonCohDevMem(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: data not aligned to granule size
    (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: data not delegatable DRAM
    (!PaIsDelegableDevMem(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: data granule not in DELEGATED state
    (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: data >= 2^48 and feat_lpa2 is FEATURE_FALSE
    ((data as int) >= (1u64 << 48) && (ImplFeatures(old_s).feat_lpa2 == RMM_FEATURE_FALSE) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rd not aligned to granule size
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rd not delegatable
    (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rd granule not in RD state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa not aligned to granule size
    (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa not in protected IPA space of target realm
    (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: target realm not in REALM_NEW state
    (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    // Failure: RTT walk stops before page level
    (RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT))
    // Failure: RTT entry not in UNASSIGNED state
    (RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    // Success: data granule is in DATA state
    (GranuleAt(new_s, data).state == DATA)
    // Success: RTT entry for ipa is in ASSIGNED state
    (RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.state == ASSIGNED)
    // Success: RTT entry RIPAS is RAM
    (RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM)
    // Success: RTT entry output address is data
    (RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.addr == data)
    // Success: RTT entry memory attribute is MEMATTR_CACHEABLE
    (RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.attr_prot == MEMATTR_CACHEABLE)
    // Success: RTT entry shareability is SHAREABILITY_INNER
    (RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.sh == SHAREABILITY_INNER)
    // Success: result is Ok
    (result.is_Ok())
}