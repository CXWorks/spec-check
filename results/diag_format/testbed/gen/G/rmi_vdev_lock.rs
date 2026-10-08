pub open spec fn rmi_vdev_lock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == RMM_FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state == VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da == RMM_FEATURE_FALSE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !(AddrIsGranuleAligned(old_s, vdev_ptr)) &&
       !(PaIsDelegable(old_s, vdev_ptr)) &&
       !(GranuleAt(old_s, vdev_ptr).state == VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}