pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok() || result.is_Err())
  && (AddrIsGranuleAligned(old_s, pdev_ptr) ==> result.is_Ok() || result.is_Err())
  && (AddrIsGranuleAligned(old_s, vdev_ptr) ==> result.is_Ok() || result.is_Err())
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs - 1)
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       result.is_Err())
    ==> GranuleAt(new_s, vdev_ptr).state == VDEV)
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == VDEV)
}