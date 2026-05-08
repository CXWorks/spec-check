```verus
pub open spec fn RMI_VDEV_COMPLETE_spec(
    old_s: S,
    new_s: S,
    rec_ptr: Address,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let rec = RecAt(old_s, rec_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    let granule_rec = GranuleAt(old_s, rec_ptr);
    let granule_vdev = GranuleAt(old_s, vdev_ptr);
    
    // Failure condition: rec_align
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: rec_bound
    (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: recv_state
    (granule_rec.state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: vdev_state
    (granule_vdev.state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: pending
    (rec.pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: owner
    (rec.owner != vdev.realm ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: vdev_id
    (rec.vdev_id_1 != vdev.vdev_id ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: comm_state
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Success conditions: all preconditions pass
    (
        AddrIsGranuleAligned(rec_ptr) &&
        PaIsDelegable(rec_ptr) &&
        granule_rec.state == REC &&
        AddrIsGranuleAligned(vdev_ptr) &&
        PaIsDelegable(vdev_ptr) &&
        granule_vdev.state == VDEV &&
        rec.pending == REC_PENDING_VDEV_REQUEST &&
        rec.owner == vdev.realm &&
        rec.vdev_id_1 == vdev.vdev_id &&
        vdev.comm_state == DEV_COMM_IDLE
    ) ==> (
        result.is_Ok() &&
        RecAt(new_s, rec_ptr).pending == REC_PENDING_VDEV_COMPLETE &&
        RecAt(new_s, rec_ptr).vdev_pa_1 == vdev_ptr &&
        VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING
    )
}
```