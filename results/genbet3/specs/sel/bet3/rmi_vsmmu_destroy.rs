pub open spec fn rmi_vsmmu_destroy_spec(rd: Address, vsmmu_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_vsmmu != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU ==> result.status == RMI_ERROR_INPUT)
  && (VsmmuIsLive(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_DEVICE)
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == GRAN_DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(new_s, rd).num_vsmmus - 1)
  && ((!(Rmm().static.feat_vsmmu != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsTrackedFine(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU) &&
       !(VsmmuIsLive(old_s, vsmmu_ptr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
}