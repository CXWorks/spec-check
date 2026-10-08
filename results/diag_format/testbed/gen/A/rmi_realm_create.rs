pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let params = RmiRealmParamsAt(old_s, params_ptr);
    let realm = RealmAt(new_s, rd);
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RealmParamsSupported(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrInRange(old_s, rd, params.rtt_base, (params.rtt_num_start as int) - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, params.rtt_base, (params.rtt_num_start as int) * (RMM_GRANULE_SIZE as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttConfigIsValid(old_s, params.s2sz as int, params.rtt_level_start as int, params.rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttsStateEqual(params.rtt_base, params.rtt_num_start as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!VmidsAreValid(old_s, params.vmid, params.aux_vmid) || !VmidsAreFree1(old_s, params.vmid, params.aux_vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (params.mecid > ImplFeatures(old_s).max_mecid ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecState(old_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiRealmParamsIsValid(old_s, params_ptr)
        && RealmParamsSupported(old_s, params)
        && !AddrInRange(old_s, rd, params.rtt_base, (params.rtt_num_start as int) - 1)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegableDram(old_s, rd)
        && GranuleAt(old_s, rd).state == DELEGATED
        && AddrIsAligned(old_s, params.rtt_base, (params.rtt_num_start as int) * (RMM_GRANULE_SIZE as int))
        && RttConfigIsValid(old_s, params.s2sz as int, params.rtt_level_start as int, params.rtt_num_start as int)
        && RttsStateEqual(params.rtt_base, params.rtt_num_start as int, DELEGATED)
        && VmidsAreValid(old_s, params.vmid, params.aux_vmid)
        && VmidsAreFree1(old_s, params.vmid, params.aux_vmid)
        && params.mecid <= ImplFeatures(old_s).max_mecid
        && MecState(old_s, params.mecid) != MEC_STATE_PRIVATE_ASSIGNED)
        ==> (result.is_Ok()
            && GranuleAt(new_s, rd).state == RD
            && realm.state == REALM_NEW
            && realm.rec_index == 0
            && RealmRttBaseEqual(realm, params.rtt_base, params.aux_rtt_base)
            && RttsStateEqual(realm.rtt_base[0], realm.rtt_num_start as int, RTT)
            && RttsAllProtectedEntriesState(new_s, realm.rtt_base[0], realm.rtt_num_start as int, UNASSIGNED)
            && RttsAllUnprotectedEntriesState(new_s, realm.rtt_base[0], realm.rtt_num_start as int, UNASSIGNED_NS)
            && RttsAllProtectedEntriesRipas(new_s, realm.rtt_base[0], realm.rtt_num_start as int, EMPTY)
            && Equal(realm.feat_lpa2, params.flags0.lpa2)
            && realm.ipa_width == params.s2sz
            && (params.hash_algo == RMI_HASH_SHA_256 ==> realm.hash_algo == HASH_SHA_256)
            && (params.hash_algo == RMI_HASH_SHA_512 ==> realm.hash_algo == HASH_SHA_512)
            && realm.measurements[0] == RimInit(new_s, realm.hash_algo, params)
            && realm.rtt_level_start == params.rtt_level_start
            && (realm.rtt_num_start as int) == (params.rtt_num_start as int)
            && RealmVmidEqual(realm, params.vmid, params.aux_vmid)
            && realm.rpv == params.rpv
            && Equal(realm.feat_da, params.flags0.da)
            && Equal(realm.feat_ats, params.flags1.ats)
            && realm.ats_plane == params.ats_plane
            && Equal(realm.rtt_tree_per_plane, params.flags1.rtt_tree_per_plane)
            && realm.num_aux_planes == params.num_aux_planes
            && (params.flags1.rtt_s2ap_encoding == RMI_S2AP_DIRECT ==> realm.rtt_s2ap_encoding == S2AP_DIRECT)
            && (params.flags1.rtt_s2ap_encoding == RMI_S2AP_INDIRECT ==> realm.rtt_s2ap_encoding == S2AP_INDIRECT)
            && (params.flags0.lfa_policy == RMI_LFA_ALLOW ==> realm.lfa_policy == LFA_ALLOW)
            && (params.flags0.lfa_policy == RMI_LFA_DISALLOW ==> realm.lfa_policy == LFA_DISALLOW)
            && realm.mecid == params.mecid
            && realm.mec_policy == MecPolicy(new_s, realm.mecid)
            && (MecState(old_s, params.mecid) == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED)
            && (MecState(old_s, params.mecid) == MEC_STATE_SHARED ==> MecMembers(new_s, params.mecid) == MecMembers(old_s, params.mecid) + 1)
            && realm.num_recs == 0
            && realm.num_vdevs == 0
            && realm.num_vsmmus == 0))
}
