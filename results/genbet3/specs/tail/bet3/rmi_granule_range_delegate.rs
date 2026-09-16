pub open spec fn rmi_granule_range_delegate_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (Rmm().dynamic.state != RMM_STATE_ACTIVE ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && (result.get_Err_0() == RMI_ERROR_INPUT || result.get_Err_0() == RMI_ERROR_TRACKING) ==> result.data.level_addr.addr identifies the PA at which the error was encountered)
  && (result.is_Err() && (result.get_Err_0() == RMI_ERROR_INPUT || result.get_Err_0() == RMI_ERROR_TRACKING) ==> result.data.level_addr.level identifies the granularity of the tracking region)
  && (result.is_Err() ==> GranulesAllState(new_s, base, out_top, GRAN_DELEGATED))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((!(Rmm().dynamic.state != RMM_STATE_ACTIVE) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranulesAllState(new_s, base, out_top, GRAN_DELEGATED))
}