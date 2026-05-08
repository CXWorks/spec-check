pub open spec fn RMI_REC_DESTROY_spec(
    old_s: S,
    new_s: S,
    rec_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let rec_align = !AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound = !PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_gran_state_pre = GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let rd_pre = RecAt(old_s, rec_ptr).owner;
    let realm_pre = RealmAt(old_s, rd_pre);
    let rec_pre = RecAt(old_s, rec_ptr);
    let rec_state = rec_pre.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC);

    let rec_gran_state_post = result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == DELEGATED;
    let rec_aux_state_post = result.is_Ok() ==> AuxStateEqual32(
        rec_pre.aux,
        RecAuxCount(rd_pre),
        DELEGATED,
    );
    let num_recs_post = result.is_Ok() ==> RealmAt(new_s, rd_pre).num_recs == realm_pre.num_recs
        - 1;

    rec_align && rec_bound && rec_gran_state_pre && rec_state && rec_gran_state_post
        && rec_aux_state_post && num_recs_post
}