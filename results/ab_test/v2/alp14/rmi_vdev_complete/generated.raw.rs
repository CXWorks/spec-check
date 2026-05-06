```verus
pub open spec fn rmi_vdev_complete_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, rec_ptr: Address, vdev_ptr: Address) -> bool {
    // Failure conditions
    let rec_align_fail = !AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound_fail = !PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let recv_state_fail = GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT);
    
    let rec = RecAt(old_s, rec_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    
    let pending_fail = rec.pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(result, RMI_ERROR_INPUT);
    let owner_fail = rec.owner != vdev.realm ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_id_fail = rec.vdev_id_1 != vdev.vdev_id ==> ResultEqual(result, RMI_ERROR_INPUT);
    let comm_state_fail = vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE);
    
    // Success conditions
    let success_pending = result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_VDEV_COMPLETE;
    let success_vdev_pa = result.is_Ok() ==> RecAt(new_s, rec_ptr).vdev_pa_1 == vdev_ptr;
    let success_comm_state = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING;
    
    rec_align_fail
    && rec_bound_fail
    && recv_state_fail
    && vdev_align_fail
    && vdev_bound_fail
    && vdev_state_fail
    && pending_fail
    && owner_fail
    && vdev_id_fail
    && comm_state_fail
    && success_pending
    && success_vdev_pa
    && success_comm_state
}
```