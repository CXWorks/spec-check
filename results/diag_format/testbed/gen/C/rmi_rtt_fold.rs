pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).level as int)))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).level as int)))
  && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
  && (AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).level as int)))
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.state == RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).state)
  && (result.is_Ok() && (RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED && RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED_NS) ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr == RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).addr)
  && (result.is_Ok() && RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).state == ASSIGNED ==> (RttMemAttrEqual(RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)), RTT_PROTECTED) && RttS2APEqual(RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)), S2AP_INDIRECT)))
  && (result.is_Ok() && RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).state == ASSIGNED_NS ==> (RttMemAttrEqual(RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)), RTT_UNPROTECTED) && RttS2APEqual(RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)), RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() && AddrIsProtected(new_s, ipa, RealmAt(new_s, rd)) ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.ripas == RttFold(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr)).ripas)
  && (result.is_Ok() ==> rtt == FoldedRttAddress(new_s, rd, ipa, level as int))
  && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == DELEGATED)
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.addr)) &&
       !(AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr == RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.ripas == RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,level - 1 as int).rtte.addr).state == GranuleAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,level - 1 as int).rtte.addr).state)
}