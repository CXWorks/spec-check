pub open spec fn rmi_rec_destroy_spec(
    result: RmiCommandReturnCode,
    rec_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let rd_pre = RecAt(old_s, rec_ptr).owner;
    let realm_pre = RealmAt(old_s, rd_pre);
    let rec_pre = RecAt(old_s, rec_ptr);
    ((!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rec_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rec_ptr).state != REC
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec_pre.state == REC_RUNNING ==> ResultEqual(
        result,
        RMI_ERROR_REC,
    )) && ((AddrIsGranuleAligned(rec_ptr) && PaIsDelegable(rec_ptr) && GranuleAt(
        old_s,
        rec_ptr,
    ).state == REC && rec_pre.state != REC_RUNNING) ==> (result == RMI_OK && GranuleAt(
        new_s,
        rec_ptr,
    ).state == DELEGATED && AuxStateEqual32(rec_pre.aux, RecAuxCount(rd_pre), DELEGATED) && RealmAt(
        new_s,
        rd_pre,
    ).num_recs == realm_pre.num_recs - 1)))
}