pub open spec fn rmi_dev_mem_unmap_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, pa: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (result.is_Ok() ==> GranulesAllState(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr, ToAddress((RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr) + RttLevelSize(new_s, level as int)), DELEGATED))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> pa == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2)) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}
