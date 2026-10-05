pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  ((!(RdIsAlignedToGranule(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(RdIsDelegablePhysicalAddress(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, rd).state == RD) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(IsRttLevelValid(old_s, RealmAt(old_s, rd), level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(RttLevelIsStartingLevel(old_s, RealmAt(old_s, rd), level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(IsIpaAligned(old_s, RealmAt(old_s, rd), ipa, level - 1 as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(IpaIsWithinIpaSpace(old_s, RealmAt(old_s, rd), ipa)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (result == RMI_ERROR_RTT(0) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level))
   && (result == RMI_ERROR_RTT(0) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level))
   && (result == RMI_ERROR_RTT(0) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level))
   && (result == RMI_ERROR_RTT(0) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level))
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == TABLE ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED))
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == TABLE && IsProtectedAddress(old_s, ipa) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).ripas == DESTROYED))
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == TABLE && !IsProtectedAddress(old_s, ipa) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS))
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == TABLE ==> GranuleAt(new_s, RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level,ipa)).state == DELEGATED))
   && (result.is_Ok() ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level == top))
   && ((RdIsAlignedToGranule(old_s, rd) &&
        RdIsDelegablePhysicalAddress(old_s, rd) &&
        GranuleAt(old_s, rd).state == RD &&
        IsRttLevelValid(old_s, RealmAt(old_s, rd), level) &&
        !(RttLevelIsStartingLevel(old_s, RealmAt(old_s, rd), level)) &&
        IsIpaAligned(old_s, RealmAt(old_s, rd), ipa, level - 1 as int) &&
        IpaIsWithinIpaSpace(old_s, RealmAt(old_s, rd), ipa))
     ==> result.is_Ok())
   && (result.is_Err()
     ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state))
   && (result.is_Err()
     ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).ripas == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).ripas))
   && (result.is_Err()
     ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state))
   && (result.is_Err()
     ==> GranuleAt(new_s, RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level,ipa)).state == GranuleAt(old_s, RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level,ipa)).state)
   && (result.is_Err()
     ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).level))
   && (!(result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED || RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS)) ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd),ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY).rtte.state)))
  )
}