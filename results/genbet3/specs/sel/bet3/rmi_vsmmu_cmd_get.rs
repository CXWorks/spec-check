pub open spec fn rmi_vsmmu_cmd_get_spec(vsmmu_ptr: Address, rd: Address, result: Result<(RmiVsmmuCmdFlags, UInt64, Address, UInt64), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RmmFeatures(old_s).feat_vsmmu != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (VsmmuAt(old_s, vsmmu_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result.status == RMI_ERROR_REALM(0))
  && ((!(RmmFeatures(old_s).feat_vsmmu != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsTracked(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(VsmmuAt(old_s, vsmmu_ptr).realm != rd) &&
       !(RealmAt(old_s, rd).state != REALM_NEW))
    ==> result.is_Ok())
}