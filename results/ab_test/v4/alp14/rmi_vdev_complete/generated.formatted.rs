pub open spec fn RMI_VDEV_COMPLETE_spec(
    old_s: S,
    new_s: S,
    rec_ptr: Address,
    vdev_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    ((!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rec_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rec_ptr).state != REC
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(vdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    RecAt(old_s, rec_ptr).pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (RecAt(old_s, rec_ptr).owner != VdevAt(old_s, vdev_ptr).realm ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (RecAt(old_s, rec_ptr).vdev_id_1 != VdevAt(old_s, vdev_ptr).vdev_id ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && ((AddrIsGranuleAligned(rec_ptr) && PaIsDelegable(rec_ptr) && GranuleAt(
        old_s,
        rec_ptr,
    ).state == REC && AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && GranuleAt(
        old_s,
        vdev_ptr,
    ).state == VDEV && RecAt(old_s, rec_ptr).pending == REC_PENDING_VDEV_REQUEST && RecAt(
        old_s,
        rec_ptr,
    ).owner == VdevAt(old_s, vdev_ptr).realm && RecAt(old_s, rec_ptr).vdev_id_1 == VdevAt(
        old_s,
        vdev_ptr,
    ).vdev_id && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE) ==> (result.is_Ok() && RecAt(
        new_s,
        rec_ptr,
    ).pending == REC_PENDING_VDEV_COMPLETE && RecAt(new_s, rec_ptr).vdev_pa_1 == vdev_ptr && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING)))
}