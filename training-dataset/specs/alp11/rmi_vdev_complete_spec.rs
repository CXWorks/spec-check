pub open spec fn rmi_vdev_complete_spec(rec_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).owner != VdevAt(old_s, vdev_ptr).realm ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).vdev_id != VdevAt(old_s, vdev_ptr).vdev_id ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RecAt(old_s, rec_ptr).inst_id_valid == RMM_TRUE && RecAt(old_s, rec_ptr).inst_id != VdevAt(old_s, vdev_ptr).inst_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_NONE)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(RecAt(old_s, rec_ptr).pending != REC_PENDING_VDEV_REQUEST) &&
       !(RecAt(old_s, rec_ptr).owner != VdevAt(old_s, vdev_ptr).realm) &&
       !(RecAt(old_s, rec_ptr).vdev_id != VdevAt(old_s, vdev_ptr).vdev_id) &&
       !((RecAt(old_s, rec_ptr).inst_id_valid == RMM_TRUE && RecAt(old_s, rec_ptr).inst_id != VdevAt(old_s, vdev_ptr).inst_id)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pending == RecAt(old_s, rec_ptr).pending)
}
