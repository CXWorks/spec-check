```rust
pub open spec fn RMI_REC_CREATE_spec(
    s_pre: S,
    s_post: S,
    rd: Address,
    rec_ptr: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let realm_pre = RealmAt(s_pre, rd);
    let realm_post = RealmAt(s_post, rd);
    let params = RmiRecParamsAt(s_pre, params_ptr);
    let rec_post = RecAt(s_post, rec_ptr);
    let rec_aux_count = RecAuxCount(rd);

    // Failure conditions - checked in order
    if !AddrIsGranuleAligned(params_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !GranuleAccessPermitted(s_pre, params_ptr, PAS_NS) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !AddrIsGranuleAligned(rec_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !PaIsDelegableDram(s_pre, rec_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if GranuleAt(s_pre, rec_ptr).state != DELEGATED {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !AddrIsGranuleAligned(rd) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !PaIsDelegable(s_pre, rd) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if GranuleAt(s_pre, rd).state != RD {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if realm_pre.state != REALM_NEW {
        return ResultEqual(result, RMI_ERROR_REALM);
    }
    if realm_pre.num_recs == (1 << ImplFeatures(s_pre).max_recs_order) - 1 {
        return ResultEqual(result, RMI_ERROR_REALM);
    }
    if RecIndex(s_pre, params.mpidr) != realm_pre.rec_index {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if params.num_aux != rec_aux_count {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !AuxAligned32(params.aux, params.num_aux) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if AuxAlias32(s_pre, rec_ptr, params.aux, params.num_aux) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !AuxStateEqual32(s_pre, params.aux, params.num_aux, DELEGATED) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }

    // Success conditions
    result.is_Ok() &&
    realm_post.rec_index == realm_pre.rec_index + 1 &&
    GranuleAt(s_post, rec_ptr).state == REC &&
    rec_post.owner == rd &&
    rec_post.attest_state == NO_ATTEST_IN_PROGRESS &&
    MpidrEqual(s_post, rec_post.mpidr, params.mpidr) &&
    rec_post.state == REC_READY &&
    (params.flags.runnable == RMI_RUNNABLE ==> rec_post.flags.runnable == RUNNABLE) &&
    (params.flags.runnable == RMI_NOT_RUNNABLE ==> rec_post.flags.runnable == NOT_RUNNABLE) &&
    rec_post.gprs[0] == params.gprs[0] &&
    rec_post.gprs[1] == params.gprs[1] &&
    rec_post.gprs[2] == params.gprs[2] &&
    rec_post.gprs[3] == params.gprs[3] &&
    rec_post.gprs[4] == params.gprs[4] &&
    rec_post.gprs[5] == params.gprs[5] &&
    rec_post.gprs[6] == params.gprs[6] &&
    rec_post.gprs[7] == params.gprs[7] &&
    (forall|i: int| 8 <= i && i < 32 ==> rec_post.gprs[i] == Zeros(64)) &&
    rec_post.pc == params.pc &&
    (params.flags.runnable == RMI_RUNNABLE ==>
        realm_post.measurements[0] == RimExtendRec(s_pre, realm_pre, params)) &&
    AuxEqual32(s_post, rec_post.aux, params.aux, rec_aux_count) &&
    AuxStateEqual32(s_post, rec_post.aux, rec_aux_count, REC_AUX) &&
    rec_post.ripas_addr == Zeros(ADDRESS_WIDTH) &&
    rec_post.ripas_top == Zeros(ADDRESS_WIDTH) &&
    rec_post.pending == REC_PENDING_NONE &&
    realm_post.num_recs == realm_pre.num_recs + 1 &&
    rec_post.gic_owner == 0
}
```