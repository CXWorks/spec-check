```rust
pub open spec fn RMI_REALM_CREATE_spec(
    s: S,
    rd: Address,
    params_ptr: Address,
) -> (result: Result<(), RmiStatusCode>, s_post: S)
{
    let params = RmiRealmParamsAt(s, params_ptr);
    let realm = RealmAt(s, rd);
    let mec_members_pre = MecMembers(s, params.mecid);
    let mec_state_pre = MecState(s, params.mecid);
    
    // Failure conditions - params validation
    if !AddrIsGranuleAligned(params_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !GranuleAccessPermitted(s, params_ptr, PAS_NS) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !RmiRealmParamsIsValid(s, params_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !RealmParamsSupported(s, params) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if AddrInRange(rd, params.rtt_base, (params.rtt_num_start - 1) * RMM_GRANULE_SIZE) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(rd) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !PaIsDelegableDram(s, rd) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rd).state != DELEGATED {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !AddrIsAligned(params.rtt_base, params.rtt_num_start * RMM_GRANULE_SIZE) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !RttConfigIsValid(s, params.s2sz, params.rtt_level_start, params.rtt_num_start) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !RttsStateEqual(params.rtt_base, params.rtt_num_start, DELEGATED) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !VmidsAreValid(s, params.vmid, params.aux_vmid) || !VmidsAreFree(s, params.vmid, params.aux_vmid) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if UInt(params.mecid) > UInt(ImplFeatures(s).max_mecid) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if MecState(s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else {
        // Success: construct updated state
        let s_post = arbitrary::<S>();
        
        // Postconditions for successful execution
        assume {
            // RD state updated
            GranuleAt(s_post, rd).state == RD &&
            
            // Realm state initialized
            RealmAt(s_post, rd).state == REALM_NEW &&
            RealmAt(s_post, rd).rec_index == 0 &&
            
            // RTT configuration
            RealmRttBaseEqual(s_post, RealmAt(s_post, rd), params.rtt_base, params.aux_rtt_base) &&
            RttsStateEqual(RealmAt(s_post, rd).rtt_base[0], RealmAt(s_post, rd).rtt_num_start, RTT) &&
            
            // RTT entry states
            RttsAllProtectedEntriesState(s_post, RealmAt(s_post, rd).rtt_base[0], RealmAt(s_post, rd).rtt_num_start, UNASSIGNED) &&
            RttsAllUnprotectedEntriesState(s_post, RealmAt(s_post, rd).rtt_base[0], RealmAt(s_post, rd).rtt_num_start, UNASSIGNED_NS) &&
            RttsAllProtectedEntriesRipas(s_post, RealmAt(s_post, rd).rtt_base[0], RealmAt(s_post, rd).rtt_num_start, EMPTY) &&
            
            // Realm feature flags and configuration
            Equal(RealmAt(s_post, rd).feat_lpa2, params.flags0.lpa2) &&
            RealmAt(s_post, rd).ipa_width == params.s2sz &&
            Equal(RealmAt(s_post, rd).hash_algo, params.hash_algo) &&
            
            // Realm measurements (RIM initialized, REMs zeroed)
            RealmAt(s_post, rd).measurements[0] == RimInit(s_post, RealmAt(s_post, rd).hash_algo, params) &&
            RealmAt(s_post, rd).measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
            RealmAt(s_post, rd).measurements[2] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
            RealmAt(s_post, rd).measurements[3] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
            RealmAt(s_post, rd).measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) &&
            
            // RTT level and number configuration
            RealmAt(s_post, rd).rtt_level_start == params.rtt_level_start &&
            RealmAt(s_post, rd).rtt_num_start == params.rtt_num_start &&
            
            // VMID configuration
            RealmVmidEqual(s_post, RealmAt(s_post, rd), params.vmid, params.aux_vmid) &&
            RealmAt(s_post, rd).rpv == params.rpv &&
            
            // Feature configuration
            Equal(RealmAt(s_post, rd).feat_da, params.flags0.da) &&
            Equal(RealmAt(s_post, rd).feat_ats, params.flags1.ats) &&
            RealmAt(s_post, rd).ats_plane == params.ats_plane &&
            Equal(RealmAt(s_post, rd).rtt_tree_per_plane, params.flags1.rtt_tree_per_plane) &&
            RealmAt(s_post, rd).num_aux_planes == params.num_aux_planes &&
            Equal(RealmAt(s_post, rd).rtt_s2ap_encoding, params.flags1.rtt_s2ap_encoding) &&
            Equal(RealmAt(s_post, rd).lfa_policy, params.flags0.lfa_policy) &&
            
            // MEC configuration
            RealmAt(s_post, rd).mecid == params.mecid &&
            Equal(RealmAt(s_post, rd).mec_policy, MecPolicy(s_post, RealmAt(s_post, rd).mecid)) &&
            (mec_state_pre == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(s_post, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED) &&
            (mec_state_pre == MEC_STATE_SHARED ==> MecMembers(s_post, params.mecid) == mec_members_pre + 1) &&
            
            // Realm counters initialized to zero
            RealmAt(