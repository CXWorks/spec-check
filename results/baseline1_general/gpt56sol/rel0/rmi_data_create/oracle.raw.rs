pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> Granule(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == data)
  && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimExtendData(new_s, Realm(new_s, rd), ipa, data, flags))
  && ((AddrIsGranuleAligned(old_s, src) &&
       PaIsDelegable(old_s, src) &&
       GranuleAccessPermitted(old_s, src, PAS_NS) &&
       AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegable(old_s, data) &&
       !(Granule(old_s, data).state != DELEGATED) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48))) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(Realm(old_s, rd).state != REALM_NEW) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, data).state == Granule(old_s, data).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr)
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
}