pub open spec fn RMI_REC_CREATE_spec(
    s: S,
    rd: Address,
    rec_ptr: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let realm = RealmAt(s, rd);
    let params = RmiRecParamsAt(s, params_ptr);
    let rec = RecAt(s, rec_ptr);

    let params_aligned = AddrIsGranuleAligned(s, params_ptr);
    let params_permitted = GranuleAccessPermitted(s, params_ptr, PAS_NS);
    let rec_aligned = AddrIsGranuleAligned(s, rec_ptr);
    let rec_bound = PaIsDelegableDram(s, rec_ptr);
    let rec_state_ok = GranuleAt(s, rec_ptr).state == DELEGATED;
    let rd_aligned = AddrIsGranuleAligned(s, rd);
    let rd_bound = PaIsDelegable(s, rd);
    let rd_state_ok = GranuleAt(s, rd).state == RD;
    let realm_state_ok = realm_pre.state == REALM_NEW;
    let num_recs_ok = realm_pre.num_recs < (pow(2, ImplFeatures(s).max_recs_order)) - 1;
    let mpidr_index_ok = RecIndex(s, params.mpidr) == realm_pre.rec_index;
    let num_aux_ok = params.num_aux == RecAuxCount(s, rd);
    let aux_aligned_ok = AuxAligned32(s, params.aux, params.num_aux);
    let aux_alias_ok = !AuxAlias32(s, rec_ptr, params.aux, params.num_aux);
    let aux_state_ok = AuxStateEqual32(s, params.aux, params.num_aux, DELEGATED);

    let failure_conditions = (!params_aligned && ResultEqual(result, RMI_ERROR_INPUT)) || (
    !params_permitted && ResultEqual(result, RMI_ERROR_INPUT)) || (!rec_aligned && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) || (!rec_bound && ResultEqual(result, RMI_ERROR_INPUT)) || (!rec_state_ok && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) || (!rd_aligned && ResultEqual(result, RMI_ERROR_INPUT)) || (!rd_bound && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) || (!rd_state_ok && ResultEqual(result, RMI_ERROR_INPUT)) || (!realm_state_ok && ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) || (!num_recs_ok && ResultEqual(result, RMI_ERROR_REALM)) || (!mpidr_index_ok && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) || (!num_aux_ok && ResultEqual(result, RMI_ERROR_INPUT)) || (!aux_aligned_ok && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) || (!aux_alias_ok && ResultEqual(result, RMI_ERROR_INPUT)) || (!aux_state_ok && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ));

    let success_conditions = params_aligned && params_permitted && rec_aligned && rec_bound
        && rec_state_ok && rd_aligned && rd_bound && rd_state_ok && realm_state_ok && num_recs_ok
        && mpidr_index_ok && num_aux_ok && aux_aligned_ok && aux_alias_ok && aux_state_ok
        && result.is_Ok() && realm.rec_index == realm_pre.rec_index + 1 && GranuleAt(
        s,
        rec_ptr,
    ).state == REC && rec.owner == rd && rec.attest_state == NO_ATTEST_IN_PROGRESS && MpidrEqual(
        s,
        rec.mpidr,
        params.mpidr,
    ) && rec.state == REC_READY && ((params.flags.runnable == RMI_RUNNABLE && rec.flags.runnable
        == RUNNABLE) || (params.flags.runnable == RMI_NOT_RUNNABLE && rec.flags.runnable
        == NOT_RUNNABLE)) && rec.gprs[0] == params.gprs[0] && rec.gprs[1] == params.gprs[1]
        && rec.gprs[2] == params.gprs[2] && rec.gprs[3] == params.gprs[3] && rec.gprs[4]
        == params.gprs[4] && rec.gprs[5] == params.gprs[5] && rec.gprs[6] == params.gprs[6]
        && rec.gprs[7] == params.gprs[7] && rec.gprs[8] == Zeros(64) && rec.gprs[9] == Zeros(64)
        && rec.gprs[10] == Zeros(64) && rec.gprs[11] == Zeros(64) && rec.gprs[12] == Zeros(64)
        && rec.gprs[13] == Zeros(64) && rec.gprs[14] == Zeros(64) && rec.gprs[15] == Zeros(64)
        && rec.gprs[16] == Zeros(64) && rec.gprs[17] == Zeros(64) && rec.gprs[18] == Zeros(64)
        && rec.gprs[19] == Zeros(64) && rec.gprs[20] == Zeros(64) && rec.gprs[21] == Zeros(64)
        && rec.gprs[22] == Zeros(64) && rec.gprs[23] == Zeros(64) && rec.gprs[24] == Zeros(64)
        && rec.gprs[25] == Zeros(64) && rec.gprs[26] == Zeros(64) && rec.gprs[27] == Zeros(64)
        && rec.gprs[28] == Zeros(64) && rec.gprs[29] == Zeros(64) && rec.gprs[30] == Zeros(64)
        && rec.gprs[31] == Zeros(64) && rec.pc == params.pc && (params.flags.runnable
        == RMI_RUNNABLE ==> realm.measurements[0] == RimExtendRec(s, realm_pre, params))
        && AuxEqual32(s, rec.aux, params.aux, RecAuxCount(s, rd)) && AuxStateEqual32(
        s,
        rec.aux,
        RecAuxCount(s, rd),
        REC_AUX,
    ) && rec.ripas_addr == Zeros(ADDRESS_WIDTH) && rec.ripas_top == Zeros(ADDRESS_WIDTH)
        && rec.pending == REC_PENDING_NONE && realm.num_recs == realm_pre.num_recs + 1
        && rec.gic_owner == 0;

    failure_conditions || success_conditions
}