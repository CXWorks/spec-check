pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> VdevAt(new_s, vdev_ptr).state == VDEV_DELEGATED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).state == VDEV_DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs - 1)
  && ((!(ImplFeatures(old_s).feat_device_assignment == FEATURE_TRUE))
    ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && ((!(align_to(GranuleSize, rd)) ||
       !(is_delegable_physical_address(rd)) ||
       !(GranuleAt(old_s, rd).state == RD))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(align_to(GranuleSize, pdev_ptr)) ||
       !(is_delegable_physical_address(pdev_ptr)) ||
       !(GranuleAt(old_s, pdev_ptr).state == PDEV))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(align_to(GranuleSize, vdev_ptr)) ||
       !(is_delegable_physical_address(vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).state == VDEV))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).owner != rd
    ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).pdev != pdev_ptr
    ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && !(VdevAt(old_s, vdev_ptr).state == VDEV_NEW || VdevAt(old_s, vdev_ptr).state == VDEV_UNLOCKED || VdevAt(old_s, vdev_ptr).state == VDEV_ERROR)
    ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).num_map != 0
    ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).vdev_id == VdevAt(old_s, vdev_ptr).vdev_id)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).tdi_id == VdevAt(old_s, vdev_ptr).tdi_id)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).vsid == VdevAt(old_s, vdev_ptr).vsid)
  && ((!(ImplFeatures(old_s).feat_device_assignment == FEATURE_TRUE) &&
       align_to(GranuleSize, rd) &&
       is_delegable_physical_address(rd) &&
       GranuleAt(old_s, rd).state == RD &&
       align_to(GranuleSize, pdev_ptr) &&
       is_delegable_physical_address(pdev_ptr) &&
       GranuleAt(old_s, pdev_ptr).state == PDEV &&
       align_to(GranuleSize, vdev_ptr) &&
       is_delegable_physical_address(vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).state == VDEV &&
       !(VdevAt(old_s, vdev_ptr).owner != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       (VdevAt(old_s, vdev_ptr).state == VDEV_NEW || VdevAt(old_s, vdev_ptr).state == VDEV_UNLOCKED || VdevAt(old_s, vdev_ptr).state == VDEV_ERROR) &&
       !(VdevAt(old_s, vdev_ptr).num_map != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
}