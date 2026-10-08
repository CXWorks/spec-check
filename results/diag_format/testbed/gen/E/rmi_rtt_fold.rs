pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa < pow2(RealmAt(old_s, rd).ipa_width as nat)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level != level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!RttIsHomogeneous(old_s, RttAt(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
  && (AddrIsAuxRef(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)).addr == rtt)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa < pow2(RealmAt(old_s, rd).ipa_width as nat))) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level != level - 1) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, RttAt(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr)) &&
       !(AddrIsAuxRef(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)).addr == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY as int).level as int)).addr)
}