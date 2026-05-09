pub open spec fn rmi_rec_create_spec(result: RmiCommandReturnCode, rd: Address, rec_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let params = RmiRecParamsAt(old_s, params_ptr);
    let rec = RecAt(new_s, rec_ptr);
    let realm = RealmAt(new_s, rd);
    
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (realm_pre.num_recs == (ToBits64(1) << ImplFeatures().max_recs_order) - 1 ==> ResultEqual(result, RMI_ERROR_REALM))
    && (RecIndex(params.mpidr) != realm_pre.rec_index ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (params.num_aux != RecAuxCount(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxAligned32(params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AuxAlias32(rec_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (
        (AddrIsGranuleAligned(params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && AddrIsGranuleAligned(rec_ptr)
        && PaIsDelegableDram(rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == DELEGATED
        && AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(old_s, rd).state == RD
        && realm_pre.state == REALM_NEW
        && realm_pre.num_recs < (ToBits64(1) << ImplFeatures().max_recs_order) - 1
        && RecIndex(params.mpidr) == realm_pre.rec_index
        && params.num_aux == RecAuxCount(rd)
        && AuxAligned32(params.aux, params.num_aux)
        && !AuxAlias32(rec_ptr, params.aux, params.num_aux)
        && AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED))
        ==> (result == RMI_SUCCESS
            && realm.rec_index == realm_pre.rec_index + 1
            && GranuleAt(new_s, rec_ptr).state == REC
            && rec.owner == rd
            && rec.attest_state == NO_ATTEST_IN_PROGRESS
            && MpidrEqual(rec.mpidr, params.mpidr)
            && rec.state == REC_READY
            && (params.flags.runnable == RMI_RUNNABLE ==> rec.flags.runnable == RUNNABLE)
            && (params.flags.runnable == RMI_NOT_RUNNABLE ==> rec.flags.runnable == NOT_RUNNABLE)
            && rec.gprs[0] == params.gprs[0]
            && rec.gprs[1] == params.gprs[1]
            && rec.gprs[2] == params.gprs[2]
            && rec.gprs[3] == params.gprs[3]
            && rec.gprs[4] == params.gprs[4]
            && rec.gprs[5] == params.gprs[5]
            && rec.gprs[6] == params.gprs[6]
            && rec.gprs[7] == params.gprs[7]
            && rec.gprs[8] == Zeros(64)
            && rec.gprs[9] == Zeros(64)
            && rec.gprs[10] == Zeros(64)
            && rec.gprs[11] == Zeros(64)
            && rec.gprs[12] == Zeros(64)
            && rec.gprs[13] == Zeros(64)
            && rec.gprs[14] == Zeros(64)
            && rec.gprs[15] == Zeros(64)
            && rec.gprs[16] == Zeros(64)
            && rec.gprs[17] == Zeros(64)
            && rec.gprs[18] == Zeros(64)
            && rec.gprs[19] == Zeros(64)
            && rec.gprs[20] == Zeros(64)
            && rec.gprs[21] == Zeros(64)
            && rec.gprs[22] == Zeros(64)
            && rec.gprs[23] == Zeros(64)
            && rec.gprs[24] == Zeros(64)
            && rec.gprs[25] == Zeros(64)
            && rec.gprs[26] == Zeros(64)
            && rec.gprs[27] == Zeros(64)
            && rec.gprs[28] == Zeros(64)
            && rec.gprs[29] == Zeros(64)
            && rec.gprs[30] == Zeros(64)
            && rec.gprs[31] == Zeros(64)
            && rec.pc == params.pc
            && (params.flags.runnable == RMI_RUNNABLE ==> realm.measurements[0] == RimExtendRec(old_s, realm_pre, params))
            && AuxEqual32(rec.aux, params.aux, RecAuxCount(rd))
            && AuxStateEqual32(new_s, rec.aux, RecAuxCount(rd), REC_AUX)
            && rec.ripas_addr == Zeros(ADDRESS_WIDTH)
            && rec.ripas_top == Zeros(ADDRESS_WIDTH)
            && rec.pending == REC_PENDING_NONE
            && realm.num_recs == realm_pre.num_recs + 1
            && rec.gic_owner == 0))
}