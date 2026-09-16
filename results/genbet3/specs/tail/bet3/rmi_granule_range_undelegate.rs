pub open spec fn rmi_granule_range_undelegate_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && result.get_Err_0() == RMI_ERROR_TRACKING ==> result.get_Err_0().data.level_addr.addr identifies the PA at which the error was encountered)
  && (result.is_Err() && result.get_Err_0() == RMI_ERROR_TRACKING ==> result.get_Err_0().data.level_addr.level identifies the granularity of the tracking region)
  && (result.is_Err() && (result.get_Err_0() == RMI_ERROR_INPUT && (RttEntryAt(new_s, RttAt(new_s, base), 0 as int).state == GRAN_DELEGATED || RttEntryAt(new_s, RttAt(new_s, base), 0 as int).state == GRAN_UNDELEGATED)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranulesAllState(new_s, base, out_top, GRAN_UNDELEGATED))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       !(result.is_Err() && result.get_Err_0() == RMI_ERROR_TRACKING) &&
       !(result.get_Err_0() == RMI_ERROR_INPUT && (RttEntryAt(new_s, RttAt(new_s, base), 0 as int).state == GRAN_DELEGATED || RttEntryAt(new_s, RttAt(new_s, base), 0 as int).state == GRAN_UNDELEGATED)))
    ==> result.is_Ok())
}