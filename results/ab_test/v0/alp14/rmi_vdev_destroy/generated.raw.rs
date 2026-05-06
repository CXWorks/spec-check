```verus
pub open spec fn RMI_VDEV_DESTROY_spec(s: S, rd: Address, pdev_ptr: Address, vdev_ptr: Address) -> bool {
  let realm_pre = RealmAt(s, rd);
  let vdev_pre = VdevAt(s, vdev_ptr);
  let pdev_pre = PdevAt(s, pdev_ptr);
  
  // Failure condition: da_supp
  (ImplFeatures_feat_da(s) != FEATURE_TRUE) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_NOT_SUPPORTED) &&
  
  // Failure conditions: rd checks
  (!AddrIsGranuleAligned(s, rd)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (!PaIsDelegable(s, rd)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (GranuleAt(s, rd).state != RD) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  
  // Failure conditions: pdev checks
  (!AddrIsGranuleAligned(s, pdev_ptr)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (!PaIsDelegable(s, pdev_ptr)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (GranuleAt(s, pdev_ptr).state != PDEV) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  
  // Failure conditions: vdev checks
  (!AddrIsGranuleAligned(s, vdev_ptr)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (!PaIsDelegable(s, vdev_ptr)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  (GranuleAt(s, vdev_ptr).state != VDEV) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_INPUT) &&
  
  // Failure conditions: vdev device checks
  (vdev_pre.realm != rd) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_DEVICE) &&
  (vdev_pre.pdev != pdev_ptr) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_DEVICE) &&
  ((vdev_pre.vdev_state != VDEV_NEW && vdev_pre.vdev_state != VDEV_UNLOCKED && vdev_pre.vdev_state != VDEV_ERROR)) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_DEVICE) &&
  (vdev_pre.num_map != 0) ==> ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_ERROR_DEVICE) &&
  
  // Success conditions
  (
    (ImplFeatures_feat_da(s) == FEATURE_TRUE) &&
    AddrIsGranuleAligned(s, rd) &&
    PaIsDelegable(s, rd) &&
    GranuleAt(s, rd).state == RD &&
    AddrIsGranuleAligned(s, pdev_ptr) &&
    PaIsDelegable(s, pdev_ptr) &&
    GranuleAt(s, pdev_ptr).state == PDEV &&
    AddrIsGranuleAligned(s, vdev_ptr) &&
    PaIsDelegable(s, vdev_ptr) &&
    GranuleAt(s, vdev_ptr).state == VDEV &&
    vdev_pre.realm == rd &&
    vdev_pre.pdev == pdev_ptr &&
    (vdev_pre.vdev_state == VDEV_NEW || vdev_pre.vdev_state == VDEV_UNLOCKED || vdev_pre.vdev_state == VDEV_ERROR) &&
    vdev_pre.num_map == 0
  ) ==> (
    ResultEqual(RMI_VDEV_DESTROY_result(s, rd, pdev_ptr, vdev_ptr), RMI_OK) &&
    GranuleAt(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), vdev_ptr).state == DELEGATED &&
    AuxStateEqual32(s, vdev_pre.aux, vdev_pre.num_aux, DELEGATED) &&
    VdevIdIsFree(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), RealmAt(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), rd), vdev_pre.vdev_id) &&
    TdiIdIsFree(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), vdev_pre.tdi_id, pdev_pre.segment_id) &&
    RealmAt(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), rd).num_vdevs == realm_pre.num_vdevs - 1 &&
    PdevAt(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), pdev_ptr).num_vdevs == pdev_pre.num_vdevs - 1 &&
    (vdev_pre.vsmmu == FEATURE_TRUE ==> VsidIsFree(s, VsmmuAt(RMI_VDEV_DESTROY_post(s, rd, pdev_ptr, vdev_ptr), vdev_pre.vsmmu_addr), vdev_pre.vsid))
  )
}
```