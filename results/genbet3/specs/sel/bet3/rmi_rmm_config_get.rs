pub open spec fn rmi_rmm_config_get_spec(cfg_ptr: Address, result: Result<RmiResult, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!Rmm().dynamic.state IN { RMM_STATE_INIT, RMM_STATE_ACTIVE} ==> result.status == RMI_ERROR_GLOBAL)
  && (!AddrIsRmiGranuleAligned(old_s, cfg_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, cfg_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> RmiRmmConfigAt(new_s, cfg_ptr).rmi_granule_size == GranuleSizeToRmi(new_s, Rmm().dynamic.rmi_granule_size))
  && (result.is_Ok() ==> RmiRmmConfigAt(new_s, cfg_ptr).tracking_region_size == TrackingRegionSizeToRmi(new_s, Rmm().dynamic.rmi_granule_size,Rmm().dynamic.tracking_region_size))
  && ((!(Rmm().dynamic.state IN { RMM_STATE_INIT, RMM_STATE_ACTIVE}) &&
       AddrIsRmiGranuleAligned(old_s, cfg_ptr) &&
       NonSecureAccessPermitted(old_s, cfg_ptr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmiRmmConfigAt(new_s, cfg_ptr).rmi_granule_size == RmiRmmConfigAt(old_s, cfg_ptr).rmi_granule_size)
  && (result.is_Err()
    ==> RmiRmmConfigAt(new_s, cfg_ptr).tracking_region_size == RmiRmmConfigAt(old_s, cfg_ptr).tracking_region_size)
}