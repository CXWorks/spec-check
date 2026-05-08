```verus
pub open spec fn RMI_REALM_CREATE_spec(s: S, rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
  let params = RmiRealmParamsAt(s, params_ptr);
  let realm = RealmAt(s, rd);
  let mec_members_pre = MecMembers(s, params.mecid);
  let mec_state_pre = MecState(s, params.mecid);
  
  // Failure conditions
  let params_align_fail = !AddrIsGranuleAligned(s, params_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
  let params_pas_fail = !GranuleAccessPermitted(s, params_ptr, PAS_NS) && ResultEqual(result, RMI_ERROR_INPUT);
  let params_valid_fail = !RmiRealmParamsIsValid(s, params_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
  let params_supp_fail = !RealmParamsSupported(s, params) && ResultEqual(result, RMI_ERROR_INPUT);
  let alias_fail = AddrInRange(s, rd, params.rtt_base, (params.rtt_num_start - 1) * RMM_GRANULE_SIZE) && ResultEqual(result, RMI_ERROR_INPUT);
  let rd_align_fail = !AddrIsGranuleAligned(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
  let rd_bound_fail = !PaIsDelegableDram(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
  let rd_state_pre_fail = GranuleAt(s, rd).state != DELEGATED && ResultEqual(result, RMI_ERROR_INPUT);
  let rtt_align_fail = !AddrIsAligned(s, params.rtt_base, params.rtt_num_start * RMM_GRANULE_SIZE) && ResultEqual(result, RMI_ERROR_INPUT);
  let rtt_num_level_fail = !RttConfigIsValid(s, params.s2sz, params.rtt_level_start, params.rtt_num_start) && ResultEqual(result, RMI_ERROR_INPUT);
  let rtt_state_pre_fail = !RttsStateEqual(s, params.rtt_base, params.rtt_num_start, DELEGATED) && ResultEqual(result, RMI_ERROR_INPUT);
  let vmid_valid_fail = (!VmidsAreValid(s, params.vmid, params.aux_vmid) || !VmidsAreFree(s, params.vmid, params.aux_vmid)) && ResultEqual(result, RMI_ERROR_INPUT);
  let mecid_bound_fail = UInt(params.mecid) > UInt(ImplFeatures(s).max_mecid) && ResultEqual(result, RMI_ERROR_INPUT);
  let mecid_state_fail = MecState(s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED && ResultEqual(result, RMI_ERROR_INPUT);
  
  // Success conditions
  let rd_state_success = result.is_Ok() ==> GranuleAt(s, rd).state == RD;
  let realm_state_success = result.is_Ok() ==> realm.state == REALM_NEW;
  let rec_index_success = result.is_Ok() ==> realm.rec_index == 0;
  let rtt_base_success = result.is_Ok() ==> RealmRttBaseEqual(s, realm, params.rtt_base, params.aux_rtt_base);
  let rtt_state_success = result.is_Ok() ==> RttsStateEqual(s, realm.rtt_base[0], realm.rtt_num_start, RTT);
  let rtte_p_states_success = result.is_Ok() ==> RttsAllProtectedEntriesState(s, realm.rtt_base[0], realm.rtt_num_start, UNASSIGNED);
  let rtte_up_states_success = result.is_Ok() ==> RttsAllUnprotectedEntriesState(s, realm.rtt_base[0], realm.rtt_num_start, UNASSIGNED_NS);
  let rtte_ripas_success = result.is_Ok() ==> RttsAllProtectedEntriesRipas(s, realm.rtt_base[0], realm.rtt_num_start, EMPTY);
  let lpa2_success = result.is_Ok() ==> realm.feat_lpa2 == params.flags0.lpa2;
  let ipa_width_success = result.is_Ok() ==> realm.ipa_width == params.s2sz;
  let hash_algo_success = result.is_Ok() ==> realm.hash_algo == params.hash_algo;
  let rim_success = result.is_Ok() ==> realm.measurements[0] == RimInit(s, params.hash_algo, params);
  let rem_success = result.is_Ok() ==> (realm.measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.measurements[2] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.measurements[3] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH));
  let rtt_level_success = result.is_Ok() ==> realm.rtt_level_start == params.rtt_level_start;
  let rtt_num_success = result.is_Ok() ==> realm.rtt_num_start == params.rtt_num_start;
  let vmid_success = result.is_Ok() ==> RealmVmidEqual(s, realm, params.vmid, params.aux_vmid);
  let rpv_success = result.is_Ok() ==> realm.rpv == params.rpv;
  let da_success = result.is_Ok() ==> realm.feat_da == params.flags0.da;
  let ats_success = result.is_Ok() ==> realm.feat_ats == params.flags1.ats;
  let ats_plane_success = result.is_Ok() ==> realm.ats_plane == params.ats_plane;
  let rtt_tree_per_plane_success = result.is_Ok() ==> realm.rtt_tree_per_plane == params.flags1.rtt_tree_per_plane;
  let num_aux_planes_success = result.is_Ok() ==> realm.num_aux_planes == params.num_aux_planes;
  let rtt_s2ap_encoding_success = result.is_Ok() ==> realm.rtt_s2ap_encoding == params.flags1.rtt_s2ap_encoding;
  let lfa_policy_success = result.is_Ok() ==> realm.lfa_policy == params.flags0.lfa_policy;
  let mecid_success = result.is_Ok() ==> realm.mecid == params.mecid;
  let mec_policy_success = result.is_Ok() ==> realm.mec_policy == MecPolicy(s, realm.mecid);
  let mecid_private_success = result.is_Ok() && mec_state_pre == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED;
  let mec_members_success = result.is_Ok() && mec_state_pre == MEC_STATE_SHARED ==> MecMembers(s, params.mecid) == mec_members_pre + 1;
  let num_recs_success = result.is_Ok() ==> realm.num_recs == 0;
  let num_vdevs_success = result.is_Ok() ==> realm.num_vdevs == 0;
  let num_vsmmus_success = result.is_Ok() ==> realm.num_vsmmus == 0;