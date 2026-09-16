pub open spec fn rmi_granule_tracking_get_spec(base: Address, top: Address, result: Result<(RmiMemCategory, RmiTrackingRegionState, Address), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((base) > rmm.static.pasz ==> result.status == RMI_ERROR_INPUT)
  && ((top) > rmm.static.pasz ==> result.status == RMI_ERROR_INPUT)
  && ((base) >= (top) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> GranulesAllTrackingRegionState(new_s, base, result.2, result.1))
  && (result.is_Ok() ==> GranulesAllMemCategory(new_s, base, result.2, result.0))
  && ((AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((base) > rmm.static.pasz) &&
       !((top) > rmm.static.pasz) &&
       !((base) >= (top)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranulesAllTrackingRegionState(new_s, base, result.2, result.1))
  && (result.is_Err()
    ==> GranulesAllMemCategory(new_s, base, result.2, result.0))
}