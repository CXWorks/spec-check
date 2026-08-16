pub open spec fn rmi_rec_create_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    rec: Address,
    params_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(
        old_s,
        rec,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Realm(
        old_s,
        rd,
    ).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM)) && (Realm(old_s, rd).num_recs
        == (1u64 << ImplFeatures(old_s).max_recs_order) - 1 ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) && (RecIndex(old_s, RecParams(old_s, params_ptr).mpidr) != Realm(old_s, rd).rec_index as int
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (RecParams(old_s, params_ptr).num_aux as int
        != RecAuxCount(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AuxAligned(
        old_s,
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AuxAlias(
        old_s,
        rec,
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AuxStateEqual(
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
        DELEGATED,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((AddrIsGranuleAligned(old_s, params_ptr)
        && PaIsDelegable(old_s, params_ptr) && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && AddrIsGranuleAligned(old_s, rec) && PaIsDelegable(old_s, rec) && Granule(
        old_s,
        rec,
    ).state == DELEGATED && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(
        old_s,
        rd,
    ).state == RD && Realm(old_s, rd).state == REALM_NEW && Realm(old_s, rd).num_recs != (1u64
        << ImplFeatures(old_s).max_recs_order) - 1 && RecIndex(
        old_s,
        RecParams(old_s, params_ptr).mpidr,
    ) == Realm(old_s, rd).rec_index as int && RecParams(old_s, params_ptr).num_aux as int
        == RecAuxCount(old_s, rd) && AuxAligned(
        old_s,
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
    ) && !AuxAlias(
        old_s,
        rec,
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
    ) && AuxStateEqual(
        RecParams(old_s, params_ptr).aux,
        RecParams(old_s, params_ptr).num_aux as int,
        DELEGATED,
    )) ==> (result.is_Ok() && Realm(new_s, rd).rec_index == Realm(old_s, rd).rec_index + 1
        && Granule(new_s, rec).state == REC && Rec(new_s, rec).owner == rd && Rec(
        new_s,
        rec,
    ).attest_state == NO_ATTEST_IN_PROGRESS && MpidrEqual(
        Rec(new_s, rec).mpidr,
        RecParams(new_s, params_ptr).mpidr,
    ) && Rec(new_s, rec).state == REC_READY && (RecParams(new_s, params_ptr).flags.runnable
        == RMI_RUNNABLE ==> Rec(new_s, rec).flags.runnable == RUNNABLE) && (RecParams(
        new_s,
        params_ptr,
    ).flags.runnable == RMI_NOT_RUNNABLE ==> Rec(new_s, rec).flags.runnable == NOT_RUNNABLE) && Rec(
        new_s,
        rec,
    ).gprs[0] == RecParams(new_s, params_ptr).gprs[0] && Rec(new_s, rec).gprs[1] == RecParams(
        new_s,
        params_ptr,
    ).gprs[1] && Rec(new_s, rec).gprs[2] == RecParams(new_s, params_ptr).gprs[2] && Rec(
        new_s,
        rec,
    ).gprs[3] == RecParams(new_s, params_ptr).gprs[3] && Rec(new_s, rec).gprs[4] == RecParams(
        new_s,
        params_ptr,
    ).gprs[4] && Rec(new_s, rec).gprs[5] == RecParams(new_s, params_ptr).gprs[5] && Rec(
        new_s,
        rec,
    ).gprs[6] == RecParams(new_s, params_ptr).gprs[6] && Rec(new_s, rec).gprs[7] == RecParams(
        new_s,
        params_ptr,
    ).gprs[7] && Rec(new_s, rec).gprs[8] == 0 && Rec(new_s, rec).gprs[9] == 0 && Rec(
        new_s,
        rec,
    ).gprs[10] == 0 && Rec(new_s, rec).gprs[11] == 0 && Rec(new_s, rec).gprs[12] == 0 && Rec(
        new_s,
        rec,
    ).gprs[13] == 0 && Rec(new_s, rec).gprs[14] == 0 && Rec(new_s, rec).gprs[15] == 0 && Rec(
        new_s,
        rec,
    ).gprs[16] == 0 && Rec(new_s, rec).gprs[17] == 0 && Rec(new_s, rec).gprs[18] == 0 && Rec(
        new_s,
        rec,
    ).gprs[19] == 0 && Rec(new_s, rec).gprs[20] == 0 && Rec(new_s, rec).gprs[21] == 0 && Rec(
        new_s,
        rec,
    ).gprs[22] == 0 && Rec(new_s, rec).gprs[23] == 0 && Rec(new_s, rec).gprs[24] == 0 && Rec(
        new_s,
        rec,
    ).gprs[25] == 0 && Rec(new_s, rec).gprs[26] == 0 && Rec(new_s, rec).gprs[27] == 0 && Rec(
        new_s,
        rec,
    ).gprs[28] == 0 && Rec(new_s, rec).gprs[29] == 0 && Rec(new_s, rec).gprs[30] == 0 && Rec(
        new_s,
        rec,
    ).gprs[31] == 0 && Rec(new_s, rec).pc == RecParams(new_s, params_ptr).pc && (RecParams(
        new_s,
        params_ptr,
    ).flags.runnable == RMI_RUNNABLE ==> Realm(new_s, rd).measurements[0] == RimExtendRec(
        old_s,
        Realm(old_s, rd),
        RecParams(new_s, params_ptr),
    )) && AuxEqual(Rec(new_s, rec).aux, RecParams(new_s, params_ptr).aux, RecAuxCount(new_s, rd))
        && AuxStateEqual(Rec(new_s, rec).aux, RecAuxCount(new_s, rd), REC_AUX) && Rec(
        new_s,
        rec,
    ).ripas_addr == 0 && Rec(new_s, rec).ripas_top == 0 && Rec(new_s, rec).host_call_pending
        == NO_HOST_CALL_PENDING && Realm(new_s, rd).num_recs == Realm(old_s, rd).num_recs + 1))
}