pub open spec fn RMI_REC_DESTROY_spec(
    s: S,
    rec_ptr: Address,
    rd_pre: Address,
    realm_pre: RmmRealm,
    rec_pre: RmmRec,
    realm: RmmRealm,
    rec: RmmRec,
) -> bool {
    let rec_align = AddrIsGranuleAligned(rec_ptr);
    let rec_bound = PaIsDelegable(rec_ptr);
    let rec_gran_state_pre = GranuleAt(s, rec_ptr).state == RmmGranuleState::REC;
    let rec_state_pre = rec_pre.state == RmmRecState::REC_RUNNING;
    let rec_gran_state_post = GranuleAt(s, rec_ptr).state == RmmGranuleState::DELEGATED;
    let rec_aux_state_post = AuxStateEqual32(
        rec_pre.aux,
        RecAuxCount(rd_pre),
        RmmGranuleState::DELEGATED,
    );
    let num_recs_post = realm.num_recs == realm_pre.num_recs - 1;

    (rec_align && rec_bound && rec_gran_state_pre && !rec_state_pre) ==> (rec_gran_state_post
        && rec_aux_state_post && num_recs_post)
}