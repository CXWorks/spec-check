pub open spec fn rmi_granule_tracking_set_spec(addr: Address, category: RmiMemCategory, state: RmiTrackingRegionState, result: Result<(RmiStatusCode, RmmTrackingRegion),>, old_s: S, new_s: S) -> bool {
  (RmmGlobal(new_s).dynamic.state != RMM_STATE_ACTIVE ==> result.0 == RMI_ERROR_GLOBAL)
  && (!(state IN { RMI_TRACKING_NONE, RMI_TRACKING_FINE, RMI_TRACKING_INTERMEDIATE, RMI_TRACKING_COARSE}) ==> result.0 == RMI_ERROR_INPUT)
  && (!AddrIsTrackingRegionAligned(old_s, addr) ==> result.0 == RMI_ERROR_INPUT)
  && ((addr) > RmmStatic(old_s).pasz ==> result.0 == RMI_ERROR_INPUT)
  && (!MemCategoryIsCompatible(old_s, category, addr) ==> result.0 == RMI_ERROR_INPUT)
  && (TrackingRegionAt(old_s, addr).state == TRACKING_RESERVED ==> result.0 == RMI_ERROR_INPUT)
  && (result.is_Ok() && !TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, addr)) && TrackingRegionIsTracked(new_s, TrackingRegionAt(new_s, addr)) ==> RmmGlobal(new_s).dynamic.num_tracked == RmmGlobal(old_s).dynamic.num_tracked + 1)
  && (result.is_Ok() && TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, addr)) && !TrackingRegionIsTracked(new_s, TrackingRegionAt(new_s, addr)) ==> RmmGlobal(new_s).dynamic.num_tracked == RmmGlobal(old_s).dynamic.num_tracked - 1)
  && (result.is_Ok() ==> result.0 == RMI_SUCCESS)
  && (result.is_Ok() ==> Equal(TrackingRegionAt(new_s, addr).state, state))
  && ((!(RmmGlobal(new_s).dynamic.state != RMM_STATE_ACTIVE) &&
       (state IN { RMI_TRACKING_NONE, RMI_TRACKING_FINE, RMI_TRACKING_INTERMEDIATE, RMI_TRACKING_COARSE}) &&
       AddrIsTrackingRegionAligned(old_s, addr) &&
       !((addr) > RmmStatic(old_s).pasz) &&
       MemCategoryIsCompatible(old_s, category, addr) &&
       !(TrackingRegionAt(old_s, addr).state == TRACKING_RESERVED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmmGlobal(new_s).dynamic.num_tracked == RmmGlobal(old_s).dynamic.num_tracked)
  && (result.is_Err()
    ==> TrackingRegionAt(new_s, addr).state == TrackingRegionAt(old_s, addr).state)
}