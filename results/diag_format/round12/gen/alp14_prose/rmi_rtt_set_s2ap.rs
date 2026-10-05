pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (RealmAt(old_s, rd).granules[rd as int].state == GRANULE_STATE_RD ==> RMI_ERROR_INPUT(result))
  && (is_delegable_physical_address(old_s, rd) ==> RMI_ERROR_INPUT(result))
  && (RealmAt(old_s, rd).granules[rd as int].state == GRANULE_STATE_RD ==> RMI_ERROR_INPUT(result))
  && (RecAt(old_s, rec_ptr).granules[rec_ptr as int].state == GRANULE_STATE_REC ==> RMI_ERROR_INPUT(result))
  && (is_delegable_physical_address(old_s, rec_ptr) ==> RMI_ERROR_INPUT(result))
  && (RecAt(old_s, rec_ptr).granules[rec_ptr as int].state == GRANULE_STATE_REC ==> RMI_ERROR_INPUT(result))
  && (RealmAt(old_s, rd).rec_state == REC_RUNNING ==> RMI_ERROR_REC(result))
  && (RecAt(old_s, rec_ptr).owner != rd ==> RMI_ERROR_REC(result))
  && ((top) <= (base) ==> RMI_ERROR_INPUT(result))
  && (RealmAt(old_s, rec_ptr).s2ap_addr != base ==> RMI_ERROR_INPUT(result))
  && ((top) > (RealmAt(old_s, rec_ptr).s2ap_top) ==> RMI_ERROR_INPUT(result))
  && ((top) % GRANULE_SIZE == 0 ==> RMI_ERROR_INPUT(result))
  && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !(base..=top) ⊆ (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).address_aligned_down..=RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).address_aligned_up) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).tree == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).s2ap_overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index ==> RMI_ERROR_RTT(result, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).level as int))
  && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !(base..=top) ⊆ (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).address_aligned_down..=RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).address_aligned_up) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).tree != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).s2ap_overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index ==> RMI_ERROR_RTT_AUX(result, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).level as int))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
  && ((!(RealmAt(old_s, rd).granules[rd as int].state == GRANULE_STATE_RD) &&
       is_delegable_physical_address(old_s, rd) &&
       !(RealmAt(old_s, rd).granules[rd as int].state == GRANULE_STATE_RD) &&
       !(RecAt(old_s, rec_ptr).granules[rec_ptr as int].state == GRANULE_STATE_REC) &&
       !is_delegable_physical_address(old_s, rec_ptr) &&
       !(RecAt(old_s, rec_ptr).granules[rec_ptr as int].state == GRANULE_STATE_REC) &&
       !(RealmAt(old_s, rd).rec_state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(RealmAt(old_s, rec_ptr).s2ap_addr != base) &&
       !((top) > (RealmAt(old_s, rec_ptr).s2ap_top)) &&
       !((top) % GRANULE_SIZE == 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
  && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE
    ==> RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid)
}