pub open spec fn rmi_granule_tracking_get_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, category: RmiMemCategory, state: RmiTrackingRegionState, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) > (rmm.static.pasz) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (rmm.static.pasz) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) >= (top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranulesAllTrackingRegionState(new_s, base, out_top, state))
  && (result.is_Ok() ==> GranulesAllMemCategory(new_s, base, out_top, category))
  && ((AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((base) > (rmm.static.pasz)) &&
       !((top) > (rmm.static.pasz)) &&
       !((base) >= (top)))
    ==> result.is_Ok())
}