```verus
pub open spec fn RMI_REALM_CREATE_spec(old_s: S, new_s: S, rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let params = RmiRealmParamsAt(old_s, params_ptr);
    let realm = RealmAt(old_s, rd);
    let mec_members_pre = MecMembers(old_s, params.mecid);
    let mec_state_pre = MecState(old_s, params.mecid);
    
    // Failure conditions
    let params_align_fail = !AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let params_pas_fail = !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let params_valid_fail = !RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let params_supp_fail = !RealmParamsSupported(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let alias_fail = AddrInRange(rd, params.rtt_base, (params.rtt_num_start - 1) * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_align_fail = !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rtt_align_fail = !AddrIsAligned(params.rtt_base, params.rtt_num_start * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rtt_num_level_fail = !RttConfigIsValid(old_s, params.s2sz, params.rtt_level_start, params.rtt_num_start) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rtt_state_fail = !RttsStateEqual(old_s, params.rtt_base, params.rtt_num_start, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vmid_valid_fail = (!VmidsAreValid(old_s, params.vmid, params.aux_vmid) || !VmidsAreFree(old_s, params.vmid, params.aux_vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let mecid_bound_fail = UInt(params.mecid) > UInt(ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let mecid_state_fail = MecState(old_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT);
    
    // Success conditions
    let success_cond = result.is_Ok() ==> (
        GranuleAt(new_s, rd).state == RD &&
        RealmAt(new_s, rd).state == REALM_NEW &&
        RealmAt(new_s, rd).rec_index == 0 &&
        RealmRttBaseEqual(new_s, RealmAt(new_s, rd), params.rtt_base, params.aux_rtt_base) &&
        RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, RTT) &&
        RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED) &&
        RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED_NS) &&
        RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, EMPTY) &&
        Equal(RealmAt(new_s, rd).feat_lpa2, params.flags0.lpa2) &&
        RealmAt(new_s, rd).ipa_width == params.s2sz &&
        Equal(RealmAt(new_s, rd).hash_algo, params.hash_algo) &&
        RealmAt(new_s, rd).measurements[0] == RimInit(old_s, params.hash_algo, params) &&
        RealmAt(new_s, rd).measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
        RealmAt(new_s, rd).measurements[2] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
        RealmAt(new_s, rd).measurements[3] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
        RealmAt(new_s, rd).measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
        RealmAt(new_s, rd).rtt_level_start == params.rtt_level_start &&
        RealmAt(new_s, rd).rtt_num_start == params.rtt_num_start &&
        RealmVmidEqual(new_s, RealmAt(new_s, rd), params.vmid, params.aux_vmid) &&
        RealmAt(new_s, rd).rpv == params.rpv &&
        Equal(RealmAt(new_s, rd).feat_da, params.flags0.da) &&
        Equal(RealmAt(new_s, rd).feat_ats, params.flags1.ats) &&
        RealmAt(new_s, rd).ats_plane == params.ats_plane &&
        Equal(RealmAt(new_s, rd).rtt_tree_per_plane, params.flags1.rtt_tree_per_plane) &&
        RealmAt(new_s, rd).num_aux_planes == params.num_aux_planes &&
        Equal(RealmAt(new_s, rd).rtt_s2ap_encoding, params.flags1.rtt_s2ap_encoding) &&
        Equal(RealmAt(new_s, rd).lfa_policy, params.flags0.lfa_policy) &&
        RealmAt(new_s, rd).mecid == params.mecid &&
        RealmAt(new_s, rd).mec_policy == MecPolicy(new_s, params.mecid) &&
        (mec_state_pre == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED) &&
        (mec_state_pre == MEC_STATE_SHARED ==> MecMembers(new_s, params.mecid) == mec_members_pre + 1) &&
        RealmAt(new_s, rd).num_recs == 0 &&
        RealmAt(new_s, rd).num_vdevs == 0 &&
        RealmAt(new_s, rd).num_vsmmus == 0
    );
    
    params_align_fail && params_pas_fail && params_valid_fail && params_supp_fail && alias_fail && 
    rd_align_fail && rd_bound_fail && rd_state_fail && rtt_align_fail && rtt_num_level_fail && 
    rtt_state_fail && vmid_valid_fail && mecid_