```verus
pub open spec fn rmi_rec_create_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    params_ptr: Address,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let params = RmiRecParamsAt(old_s, params_ptr);
    let rec = RecAt(new_s, rec_ptr);
    
    // Failure conditions - input validation
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure conditions - realm and REC state validation
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (realm_pre.num_recs == (1 << ImplFeatures(old_s).max_recs_order) - 1 ==> 
        ResultEqual(result, RMI_ERROR_REALM))
    
    // Failure conditions - REC parameters validation
    && (RecIndex(old_s, params.mpidr) != realm_pre.rec_index ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    && (params.num_aux != RecAuxCount(old_s, rd) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxAligned32(old_s, params.aux, params.num_aux) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    && (AuxAlias32(old_s, rec_ptr, params.aux, params.num_aux) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Success conditions
    && (result.is_Ok() ==>
        // REC index advancement
        realm.rec_index == realm_pre.rec_index + 1
        // REC granule state
        && GranuleAt(new_s, rec_ptr).state == REC
        // REC ownership
        && rec.owner == rd
        // Attestation state
        && rec.attest_state == NO_ATTEST_IN_PROGRESS
        // MPIDR matching
        && MpidrEqual(old_s, rec.mpidr, params.mpidr)
        // REC state
        && rec.state == REC_READY
        // Runnable flag
        && ((params.flags.runnable == RMI_RUNNABLE ==> rec.flags.runnable == RUNNABLE)
            && (params.flags.runnable == RMI_NOT_RUNNABLE ==> rec.flags.runnable == NOT_RUNNABLE))
        // GPRs (0-7 copied, 8-31 zeroed)
        && rec.gprs[0] == params.gprs[0]
        && rec.gprs[1] == params.gprs[1]
        && rec.gprs[2] == params.gprs[2]
        && rec.gprs[3] == params.gprs[3]
        && rec.gprs[4] == params.gprs[4]
        && rec.gprs[5] == params.gprs[5]
        && rec.gprs[6] == params.gprs[6]
        && rec.gprs[7] == params.gprs[7]
        && (forall i: int :: 8 <= i && i < 32 ==> rec.gprs[i] == Zeros(64))
        // PC
        && rec.pc == params.pc
        // RIM extension (when runnable)
        && (params.flags.runnable == RMI_RUNNABLE ==>
            realm.measurements[0] == RimExtendRec(old_s, realm_pre, params))
        // Aux matching and state
        && AuxEqual32(old_s, rec.aux, params.aux, RecAuxCount(old_s, rd))
        && AuxStateEqual32(old_s, rec.aux, RecAuxCount(old_s, rd), REC_AUX)
        // RIPAS initialization
        && rec.ripas_addr == Zeros(ADDRESS_WIDTH)
        && rec.ripas_top == Zeros(ADDRESS_WIDTH)
        // Pending state
        && rec.pending == REC_PENDING_NONE
        // REC count increment
        && realm.num_recs == realm_pre.num_recs + 1
        // GIC owner
        && rec.gic_owner == 0u16
    )
}
```