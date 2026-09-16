pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result == RMI_ERROR_INPUT)
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> result == RMI_ERROR_INPUT)
  && (result == RMI_ERROR_RTT && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level && top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (result == RMI_ERROR_RTT && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_TABLE ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level && top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (result == RMI_ERROR_RTT && RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)) ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level.level == level && top == ipa))
  && (result == RMI_ERROR_RTT && AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && (result.is_Ok() ==> result == RMI_SUCCESS)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_DESTROYED)
  && (result.is_Ok() && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_UNMAPPED_NS)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GRAN_DELEGATED)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
}