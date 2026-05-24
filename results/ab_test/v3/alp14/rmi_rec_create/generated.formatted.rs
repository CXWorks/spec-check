```verus
pub open spec fn rmi_rec_create_spec(result: Result<(), RmiStatusCode>, rd: Address, rec_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    // Failure conditions
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(params_ptr, PAS_NS) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != DELEGATED ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REALM))
    && (RealmAt(old_s, rd).num_recs == (1 << ImplFeaturesMaxRecsOrder(old_s)) - 1 ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REALM))
    && (RecIndex(RmiRecParamsAt(old_s, params_ptr).mpidr) != RealmAt(old_s, rd).rec_index ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (RmiRecParamsAt(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AuxAligned32(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (AuxAlias32(old_s, rec_ptr, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AuxStateEqual32(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux, DELEGATED) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // Success conditions
    && (result.is_Ok() ==>
        RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index + 1
        && GranuleAt(new_s, rec_ptr).state == REC
        && RecAt(new_s, rec_ptr).owner == rd
        && RecAt(new_s, rec_ptr).attest_state == NO_ATTEST_IN_PROGRESS
        && MpidrEqual(RecAt(new_s, rec_ptr).mpidr, RmiRecParamsAt(old_s, params_ptr).mpidr)
        && RecAt(new_s, rec_ptr).state == REC_READY
        && (RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == RUNNABLE)
        && (RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_NOT_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == NOT_RUNNABLE)
        && RecAt(new_s, rec_ptr).gprs[0] == RmiRecParamsAt(old_s, params_ptr).gprs[0]
        && RecAt(new_s, rec_ptr).gprs[1] == RmiRecParamsAt(old_s, params_ptr).gprs[1]
        && RecAt(new_s, rec_ptr).gprs[2] == RmiRecParamsAt(old_s, params_ptr).gprs[2]
        && RecAt(new_s, rec_ptr).gprs[3] == RmiRecParamsAt(old_s, params_ptr).gprs[3]
        && RecAt(new_s, rec_ptr).gprs[4] == RmiRecParamsAt(old_s, params_ptr).gprs[4]
        && RecAt(new_s, rec_ptr).gprs[5] == RmiRecParamsAt(old_s, params_ptr).gprs[5]
        && RecAt(new_s, rec_ptr).gprs[6] == RmiRecParamsAt(old_s, params_ptr).gprs[6]
        && RecAt(new_s, rec_ptr).gprs[7] == RmiRecParamsAt(old_s, params_ptr).gprs[7]
        && RecAt(new_s, rec_ptr).gprs[8] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[9] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[10] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[11] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[12] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[13] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[14] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[15] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[16] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[17] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[18] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[19] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[20] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[21] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[22] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[23] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[24] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[25] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[26] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[27] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[28] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[29] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[30] == Zeros(64)
        && RecAt(new_s, rec_ptr).gprs[31] == Zeros(64)
        && RecAt(new_s, rec_ptr).pc == RmiRecParamsAt(old_s, params_ptr).pc
        && (RmiRecParamsAt