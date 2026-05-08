```verus
pub open spec fn rmi_realm_create_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    params_ptr: Address
) -> bool {
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
    let rd_state_success = result.is_Ok() ==> GranuleAt(new_s, rd).state == RD;
    let realm_state_success = result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW;
    let rec_index_success = result.is_Ok() ==> RealmAt(new_s, rd).rec_index == 0;
    let rtt_base_success = result.is_Ok() ==> RealmRttBaseEqual(new_s, RealmAt(new_s, rd), params.rtt_base, params.aux_rtt_base);
    let rtt_state_success = result.is_Ok() ==> RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, RTT);
    let rtte_p_states_success = result.is_Ok() ==> RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED);
    let rtte_up_states_success = result.is_Ok() ==> RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED_NS);
    let rtte_ripas_success = result.is_Ok() ==> RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, EMPTY);
    let lpa2_success = result.is_Ok() ==> RealmAt(new_s, rd).feat_lpa2 == params.flags0.lpa2;
    let ipa_width_success = result.is_Ok() ==> RealmAt(new_s, rd).ipa_width == params.s2sz;
    let hash_algo_success = result.is_Ok() ==> RealmAt(new_s, rd).hash_algo == params.hash_algo;
    let rim_success = result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimInit(new_s, params.hash_algo, params);
    let rem_success = result.is_Ok() ==> (RealmAt(new_s, rd).measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && RealmAt(new_s, rd).measurements[2] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && RealmAt(new_s, rd).measurements[3] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && RealmAt(new_s, rd).measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH));
    let rtt_level_success = result.is_Ok() ==> RealmAt(new_s, rd).rtt_level_start == params.rtt_level_start;
    let rtt_num_success = result.is_Ok() ==> RealmAt(new_s, rd).rtt_num_start == params.rtt_num_start;
    let vmid_success = result.is_Ok() ==> RealmVmidEqual(new_s, RealmAt(new_s, rd), params.vmid, params.aux_vmid);
    let rpv_success = result.is_Ok() ==> RealmAt(new_s, rd).rpv == params.rpv;
    let da_success = result.is_Ok() ==> RealmAt(new_s, rd).feat_da == params.flags0.da;
    let ats_success = result.is_Ok() ==> RealmAt(new_s, rd).feat_ats == params.flags1.ats;
    let ats_plane_success = result.is_Ok() ==> RealmAt(new_s, rd).ats_plane == params.ats_plane;
    let rtt_tree_per_plane_success = result.is_Ok() ==> RealmAt(new_s, rd).rtt_tree_per_plane == params.flags1.rtt_tree_per_plane;
    let num_aux_planes_success = result.is_Ok() ==> RealmAt(new_s, rd).num_aux_planes == params.num_aux_planes;
    let rtt_s2ap_encoding_success = result.is_Ok() ==> RealmAt(new_s, rd).rtt_s2ap_encoding == params.flags1.rtt_s2ap_encoding;
    let lfa_policy_success = result.is_