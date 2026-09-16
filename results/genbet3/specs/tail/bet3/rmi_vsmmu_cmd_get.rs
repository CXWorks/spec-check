pub open spec fn rmi_vsmmu_cmd_get_spec(vsmmu_ptr: Address, rd: Address, result: Result<(), RmiStatusCode>, flags: RmiVsmmuCmdFlags, vsid: Bits64, msi_addr: Address, msi_data: Bits64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_vsmmu != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VsmmuAt(old_s, vsmmu_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && ((!(Rmm().static.feat_vsmmu != FEATURE_TRUE) &&
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