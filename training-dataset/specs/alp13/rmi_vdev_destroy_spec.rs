pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_NEW && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).num_map != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, VdevAt(new_s, vdev_ptr).num_aux as int, DELEGATED))
  && (result.is_Ok() ==> VdevIdIsFree(new_s, RealmAt(new_s, rd), VdevAt(new_s, vdev_ptr).vdev_id))
  && (result.is_Ok() ==> TdiIdIsFree(new_s, VdevAt(new_s, vdev_ptr).tdi_id, PdevAt(new_s, pdev_ptr).segment_id))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs - 1)
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vsmmu == FEATURE_TRUE ==> VsidIsFree(new_s,  VsmmuAt(new_s, VdevAt(new_s, vdev_ptr).vsmmu_addr), VdevAt(new_s, vdev_ptr).vsid))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_NEW && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR)) &&
       !(VdevAt(old_s, vdev_ptr).num_map != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
}
