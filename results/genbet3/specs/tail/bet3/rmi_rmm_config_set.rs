pub open spec fn rmi_rmm_config_set_spec(cfg_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().dynamic.state != RMM_STATE_INIT ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (!AddrIsRmiGranuleAligned(old_s, cfg_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, cfg_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmmConfigIsSupported(old_s, RmmConfigAt(old_s, cfg_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rmm().dynamic.num_tracked != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Rmm().dynamic.rmi_granule_size == GranuleSizeFromRmi(new_s, RmmConfigAt(new_s, cfg_ptr).rmi_granule_size))
  && (result.is_Ok() ==> Rmm().dynamic.tracking_region_size == TrackingRegionSizeFromRmi(new_s, RmmConfigAt(new_s, cfg_ptr).rmi_granule_size, RmmConfigAt(new_s, cfg_ptr).tracking_region_size))
  && ((!(Rmm().dynamic.state != RMM_STATE_INIT) &&
       AddrIsRmiGranuleAligned(old_s, cfg_ptr) &&
       NonSecureAccessPermitted(old_s, cfg_ptr) &&
       RmmConfigIsSupported(old_s, RmmConfigAt(old_s, cfg_ptr)) &&
       !(Rmm().dynamic.num_tracked != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rmm().dynamic.rmi_granule_size == Rmm().dynamic.rmi_granule_size)
  && (result.is_Err()
    ==> Rmm().dynamic.tracking_region_size == Rmm().dynamic.tracking_region_size)
}