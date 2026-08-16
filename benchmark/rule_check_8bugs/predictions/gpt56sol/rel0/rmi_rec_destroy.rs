pub open spec fn rmi_rec_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rec_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(
        old_s,
        rec_ptr,
    ).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AddrIsGranuleAligned(
        old_s,
        rec_ptr,
    ) && PaIsDelegable(old_s, rec_ptr) && Granule(old_s, rec_ptr).state == REC && Rec(
        old_s,
        rec_ptr,
    ).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC)) && (AddrIsGranuleAligned(
        old_s,
        rec_ptr,
    ) && PaIsDelegable(old_s, rec_ptr) && Granule(old_s, rec_ptr).state == REC && Rec(
        old_s,
        rec_ptr,
    ).state != REC_RUNNING ==> result.is_Ok() && Granule(new_s, rec_ptr).state == DELEGATED
        && AuxStateEqual(
        Rec(old_s, rec_ptr).aux,
        RecAuxCount(old_s, Rec(old_s, rec_ptr).owner),
        DELEGATED,
    ) && Realm(new_s, Rec(old_s, rec_ptr).owner).num_recs == Realm(
        old_s,
        Rec(old_s, rec_ptr).owner,
    ).num_recs - 1)
}