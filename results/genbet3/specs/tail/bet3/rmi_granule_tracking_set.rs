pub open spec fn rmi_granule_tracking_set_spec(addr: Address, category: RmiMemCategory, state: RmiTrackingRegionState, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().dynamic.state != RMM_STATE_ACTIVE ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (!(state == RMI_TRACKING_NONE || state == RMI_TRACKING_FINE || state == RMI_TRACKING_INTERMEDIATE || state == RMI_TRACKING_COARSE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsTrackingRegionAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((addr) > Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MemCategoryIsCompatible(old_s, category, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (TrackingRegionAt(old_s, addr).state == TRACKING_RESERVED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && !TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, addr)) && TrackingRegionIsTracked(new_s, TrackingRegionAt(new_s, addr)) ==> Rmm().dynamic.num_tracked == Rmm(old_s).dynamic.num_tracked + 1)
  && (result.is_Ok() && TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, addr)) && !TrackingRegionIsTracked(new_s, TrackingRegionAt(new_s, addr)) ==> Rmm().dynamic.num_tracked == Rmm(old_s).dynamic.num_tracked - 1)
  && (result.is_Ok() ==> Equal(TrackingRegionAt(new_s, addr).state, state))
  && ((!(Rmm().dynamic.state != RMM_STATE_ACTIVE) &&
       (state == RMI_TRACKING_NONE || state == RMI_TRACKING_FINE || state == RMI_TRACKING_INTERMEDIATE || state == RMI_TRACKING_COARSE) &&
       AddrIsTrackingRegionAligned(old_s, addr) &&
       !((addr) > Rmm().static_.pasz) &&
       MemCategoryIsCompatible(old_s, category, addr) &&
       !(TrackingRegionAt(old_s, addr).state == TRACKING_RESERVED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> TrackingRegionAt(new_s, addr).state == TrackingRegionAt(old_s, addr).state)
  && (result.is_Err()
    ==> Rmm().dynamic.num_tracked == Rmm(old_s).dynamic.num_tracked)
}