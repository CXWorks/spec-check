pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (is_aligned_to(GranuleSize, rd) ==> result.is_Ok())
  && (is_delegable_physical_address(rd) ==> result.is_Ok())
  && (GranuleAt(rd).state == RD ==> result.is_Ok())
  && (is_aligned_to(GranuleSize, rec_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(rec_ptr) ==> result.is_Ok())
  && (GranuleAt(rec_ptr).state == REC ==> result.is_Ok())
  && (RecAt(rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(rec_ptr).owner != RealmAt(rd).id ==> ResultEqual(result, RMI_ERROR_REC))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top > RecAt(rec_ptr).ripas_top ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_aligned_to(RttEntrySize(RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), base) && RecAt(rec_ptr).ripas_value != RttAt(RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtte.ripas ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (is_aligned_to(GranuleSize, top) ==> result.is_Ok())
  && (base == RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)) && RecAt(rec_ptr).ripas_value != RttAt(RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtte.ripas ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(rec_ptr).ripas_value ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_addr == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))))
  && (result.is_Ok() ==> out_top == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))))
  && ((!(is_aligned_to(GranuleSize, rd)) ||
       !(is_delegable_physical_address(rd)) ||
       !(GranuleAt(rd).state == RD) ||
       !(is_aligned_to(GranuleSize, rec_ptr)) ||
       !(is_delegable_physical_address(rec_ptr)) ||
       !(GranuleAt(rec_ptr).state == REC) ||
       !(RecAt(rec_ptr).state == REC_RUNNING) ||
       !(RecAt(rec_ptr).owner != RealmAt(rd).id) ||
       !(top <= base) ||
       !(base != RecAt(rec_ptr).ripas_addr) ||
       !(top > RecAt(rec_ptr).ripas_top) ||
       is_aligned_to(RttEntrySize(RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), base) ||
       !(RecAt(rec_ptr).ripas_value != RttAt(RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtte.ripas) ||
       !(is_aligned_to(GranuleSize, top)) ||
       !(base == RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))) ||
       !(RecAt(rec_ptr).ripas_value != RttAt(RttWalk(new_s, RealmAt(new_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtte.ripas) ||
       !(RttWalk(old_s, RealmAt(old_s, rd),RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(rec_ptr).ripas_value))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
}