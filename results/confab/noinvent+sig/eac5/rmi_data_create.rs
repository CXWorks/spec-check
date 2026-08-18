pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, src)
        && PaIsDelegable(old_s, src)
        && GranuleAccessPermitted(old_s, src, PAS_NS)
        && AddrIsGranuleAligned(old_s, data)
        && PaIsDelegable(old_s, data)
        && Granule(old_s, data).state == DELEGATED
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && Realm(old_s, rd).state == REALM_NEW
        && RttWalk(old_s, rd, ipa).level >= RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, rd, ipa).rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && Granule(new_s, data).state == DATA
            && RttWalk(new_s, rd, ipa).rtte.state == ASSIGNED
            && RttWalk(new_s, rd, ipa).rtte.ripas == RAM
            && RttWalk(new_s, rd, ipa).rtte.addr == data
            && Realm(new_s, rd).measurements[0] == RimExtendData(old_s, Realm(old_s, rd), ipa, data, flags)))
}