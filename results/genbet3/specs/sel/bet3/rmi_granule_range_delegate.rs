pub open spec fn rmi_granule_range_delegate_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (Rmm().dynamic.state != RMM_STATE_ACTIVE ==> result.status == RMI_ERROR_GLOBAL)
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (result.status == RMI_SUCCESS ==> GranulesAllState(new_s, base, out_top, DELEGATED))
  && (result.status == RMI_SUCCESS ==> result.status == RMI_SUCCESS)
  && ((!(Rmm().dynamic.state != RMM_STATE_ACTIVE) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)))
    ==> result.status == RMI_SUCCESS)
}