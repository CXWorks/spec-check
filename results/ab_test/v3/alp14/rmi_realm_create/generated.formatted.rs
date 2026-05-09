pub open spec fn rmi_realm_create_spec(result: RmiCommandReturnCode, rd: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    let params = RmiRealmParamsAt(old_s, params_ptr);
    let realm = RealmAt(old_s, rd);
    let mec_state_pre = MecState(old_s, params.mecid);
    let mec_members_pre = MecMembers(old_s, params.mecid);
    
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RealmParamsSupported(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrInRange(rd, params.rtt_base, (params.rtt_num_start - 1) * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(params.rtt_base, params.rtt_num_start * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttConfigIsValid(old_s, params.s2sz, params.rtt_level_start, params.rtt_num_start) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttsStateEqual(old_s, params.rtt_base, params.rtt_num_start, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!VmidsAreValid(old_s, params.vmid, params.aux_vmid) || !VmidsAreFree(old_s, params.vmid, params.aux_vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(params.mecid) > UInt(ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (mec_state_pre == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (
        (AddrIsGranuleAligned(params_ptr)
         && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
         && RmiRealmParamsIsValid(old_s, params_ptr)
         && RealmParamsSupported(old_s, params)
         && !AddrInRange(rd, params.rtt_base, (params.rtt_num_start - 1) * RMM_GRANULE_SIZE)
         && AddrIsGranuleAligned(rd)
         && PaIsDelegableDram(old_s, rd)
         && GranuleAt(old_s, rd).state == DELEGATED
         && AddrIsAligned(params.rtt_base, params.rtt_num_start * RMM_GRANULE_SIZE)
         && RttConfigIsValid(old_s, params.s2sz, params.rtt_level_start, params.rtt_num_start)
         && RttsStateEqual(old_s, params.rtt_base, params.rtt_num_start, DELEGATED)
         && VmidsAreValid(old_s, params.vmid, params.aux_vmid)
         && VmidsAreFree(old_s, params.vmid, params.aux_vmid)
         && UInt(params.mecid) <= UInt(ImplFeatures(old_s).max_mecid)
         && mec_state_pre != MEC_STATE_PRIVATE_ASSIGNED)
        ==> (
            result.is_Ok()
            && GranuleAt(new_s, rd).state == RD
            && RealmAt(new_s, rd).state == REALM_NEW
            && RealmAt(new_s, rd).rec_index == 0
            && RealmRttBaseEqual(new_s, RealmAt(new_s, rd), params.rtt_base, params.aux_rtt_base)
            && RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, RTT)
            && RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED)
            && RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED_NS)
            && RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, EMPTY)
            && RealmAt(new_s, rd).feat_lpa2 == params.flags0.lpa2
            && RealmAt(new_s, rd).ipa_width == params.s2sz
            && RealmAt(new_s, rd).hash_algo == params.hash_algo
            && RealmAt(new_s, rd).measurements[0] == RimInit(old_s, params.hash_algo, params)
            && RealmAt(new_s, rd).measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH)
            && RealmAt(new_s, rd).measurements[2] == Zeros(RMM_REALM_MEASUREMENT_WIDTH)
            && RealmAt(new_s, rd).measurements[3] == Zeros(RMM_REALM_MEASUREMENT_WIDTH)
            && RealmAt(new_s, rd).measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH)
            && RealmAt(new_s, rd).rtt_level_start == params.rtt_level_start
            && RealmAt(new_s, rd).rtt_num_start == params.rtt_num_start
            && RealmVmidEqual(new_s, RealmAt(new_s, rd), params.vmid, params.aux_vmid)
            && RealmAt(new_s, rd).rpv == params.rpv
            && RealmAt(new_s, rd).feat_da == params.flags0.da
            && RealmAt(new_s, rd).feat_ats == params.flags1.ats
            && RealmAt(new_s, rd).ats_plane == params.ats_plane
            && RealmAt(new_s, rd).rtt_tree_per_plane == params.flags1.rtt_tree_per_plane
            && RealmAt(new_s, rd).num_aux_planes == params.num_aux_planes
            && RealmAt(new_s, rd).rtt_s2ap_encoding == params.flags1.rtt_s2ap_encoding
            && RealmAt(new_s, rd).lfa_policy == params.flags0.lfa_policy
            && RealmAt(new_s, rd).mecid == params.mecid
            && RealmAt(new_s, rd).mec_policy == MecPolicy(old_s, params.mecid)
            && (mec_state_pre ==