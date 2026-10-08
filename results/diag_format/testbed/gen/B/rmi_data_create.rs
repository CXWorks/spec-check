pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let rtte = RttEntryAt(RttAt(walk.rtt_addr), entry_idx);
    let realm = RealmAt(new_s, rd);
    let rtte_new = RttEntryAt(RttAt(RttAt(new_s, walk.rtt_addr)), entry_idx);
    (!AddrIsGranuleAligned(src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (data >= pow2(48) && realm_pre.feat_lpa2 == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (walk.level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
    && (result.is_Ok() ==> GranuleContentsCopied(old_s, src, data))
    && (result.is_Ok() ==> rtte_new.state == ASSIGNED)
    && (result.is_Ok() ==> rtte_new.ripas == RAM)
    && (result.is_Ok() ==> rtte_new.addr == data)
    && (result.is_Ok() ==> rtte_new.attr_prot == MEMATTR_CACHEABLE)
    && (result.is_Ok() ==> rtte_new.sh == SHAREABILITY_INNER)
    && (result.is_Ok() ==> RealmRim(realm) == RimExtendData(realm_pre, ipa, data, flags))
}