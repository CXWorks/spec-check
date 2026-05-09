pub open spec fn rmi_vdev_complete_spec(
    result: RmiCommandReturnCode,
    rec_ptr: Address,
    vdev_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let rec = RecAt(rec_ptr);
    let vdev = VdevAt(vdev_ptr);
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rec_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rec_ptr).state != REC
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(vdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    rec.pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec.owner
        != vdev.realm ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec.vdev_id_1 != vdev.vdev_id
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (vdev.comm_state != DEV_COMM_IDLE
        ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (AddrIsGranuleAligned(rec_ptr)
        && PaIsDelegable(rec_ptr) && GranuleAt(old_s, rec_ptr).state == REC && AddrIsGranuleAligned(
        vdev_ptr,
    ) && PaIsDelegable(vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV && rec.pending
        == REC_PENDING_VDEV_REQUEST && rec.owner == vdev.realm && rec.vdev_id_1 == vdev.vdev_id
        && vdev.comm_state == DEV_COMM_IDLE ==> result == RMI_SUCCESS && RecAt(
        new_s,
        rec_ptr,
    ).pending == REC_PENDING_VDEV_COMPLETE && RecAt(new_s, rec_ptr).vdev_pa_1 == vdev_ptr && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING)
}