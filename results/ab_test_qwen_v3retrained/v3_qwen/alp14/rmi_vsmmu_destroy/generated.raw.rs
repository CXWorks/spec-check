pub open spec fn rmi_vsmmu_destroy_spec(rd: Address, vsmmu_ptr: Address, result: Result<RmiCommandReturnCode, _>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VsmmuIsLive(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus - 1)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegable(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != VSMMU) &&
       !(VsmmuIsLive(old_s, vsmmu_ptr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
}