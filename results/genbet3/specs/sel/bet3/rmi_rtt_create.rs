pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result.status == RMI_ERROR_INPUT)
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rtt) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsPopulatedConventional(old_s, rtt) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, rtt) ==> (result.status == RMI_ERROR_TRACKING && result.data.level_addr.level == TrackingToRmiResult(old_s, rtt) && result.data.level_addr.addr == AddrShiftRmiGranule(old_s, rtt)))
  && (GranuleAt(old_s, rtt).state != GRAN_DELEGATED ==> result.status == RMI_ERROR_INPUT)
  && ((rtt) >= 2^(Rmm::static_.max_oaddr_width) ==> result.status == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_TABLE ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == GRAN_RTT)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == rtt)
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_TABLE)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttAllEntriesRipas(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))
  && (result.is_Ok() ==> RttAllEntriesState(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_UNMAPPED_NS ==> RttAllEntriesContiguous(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr, level as int))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_UNMAPPED_NS ==> RttAllEntriesMemAttr(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttAllEntriesS2AP(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte))
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       AddrIsRmiGranuleAligned(old_s, rtt) &&
       PaIsPopulatedConventional(old_s, rtt) &&
       PaIsTrackedFine(old_s, rtt) &&
       !(GranuleAt(old_s, rtt).state != GRAN_DELEGATED) &&
       !((rtt) >= 2^(Rmm::static_.max_oaddr_width)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}