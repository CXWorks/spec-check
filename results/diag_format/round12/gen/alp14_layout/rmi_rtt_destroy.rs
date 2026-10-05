pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && (PaIsDelegable(old_s, rd) ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int) ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level,ipa)))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level,ipa)))))
  && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT(level as int)) && (top == ipa))))
  && (AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level as int)))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state == UNASSIGNED)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.ripas == DESTROYED)
  && (result.is_Ok() && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level,ipa))
  && ((!(AddrIsGranuleAligned(old_s, rd)) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state != TABLE) &&
       !(RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.addr))) &&
       !(AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.state)
  && (!(result.is_Ok() && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as RttTree).rtte.ripas)
}