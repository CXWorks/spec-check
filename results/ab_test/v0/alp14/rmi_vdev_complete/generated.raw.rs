```verus
pub open spec fn RMI_VDEV_COMPLETE_spec(s: S, rec_ptr: Address, vdev_ptr: Address) -> bool {
    let rec = RecAt(s, rec_ptr);
    let vdev = VdevAt(s, vdev_ptr);
    let g_rec = GranuleAt(s, rec_ptr);
    let g_vdev = GranuleAt(s, vdev_ptr);
    
    // Failure conditions
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rec_ptr) ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (g_rec.state != REC ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (g_vdev.state != VDEV ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (rec.pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (rec.owner != vdev.realm ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (rec.vdev_id_1 != vdev.vdev_id ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_INPUT)) &&
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(RMI_VDEV_COMPLETE_result(s, rec_ptr, vdev_ptr), RMI_ERROR_DEVICE)) &&
    
    // Success conditions (when all preconditions are satisfied)
    ((AddrIsGranuleAligned(rec_ptr) && PaIsDelegable(rec_ptr) && g_rec.state == REC &&
      AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && g_vdev.state == VDEV &&
      rec.pending == REC_PENDING_VDEV_REQUEST && rec.owner == vdev.realm &&
      rec.vdev_id_1 == vdev.vdev_id && vdev.comm_state == DEV_COMM_IDLE) ==>
     (RecAt(s, rec_ptr).pending == REC_PENDING_VDEV_COMPLETE &&
      RecAt(s, rec_ptr).vdev_pa_1 == vdev_ptr &&
      VdevAt(s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}
```