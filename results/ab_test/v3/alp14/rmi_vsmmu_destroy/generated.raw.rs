pub open spec fn rmi_vsmmu_destroy_spec(result: Result<(), RmiStatusCode>, rd: Address, vsmmu_ptr: Address, old_s: S, new_s: S) -> bool {
  (!ImplFeatures_feat_da_eq_true(old_s) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VsmmuIsLive(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (
    (ImplFeatures_feat_da_eq_true(old_s) 
      && AddrIsGranuleAligned(rd) 
      && PaIsDelegable(rd) 
      && GranuleAt(old_s, rd).state == RD
      && AddrIsGranuleAligned(vsmmu_ptr) 
      && PaIsDelegable(vsmmu_ptr) 
      && GranuleAt(old_s, vsmmu_ptr).state == VSMMU
      && !VsmmuIsLive(old_s, vsmmu_ptr))
    ==> (result.is_Ok() 
      && GranuleAt(new_s, vsmmu_ptr).state == DELEGATED
      && RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus - 1)
  )
}