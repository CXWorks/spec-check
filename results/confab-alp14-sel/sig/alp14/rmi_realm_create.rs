pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let params = RmiRealmParamsAt(old_s, params_ptr);
    let realm = RealmAt(new_s, rd);
    let mec_members_pre = MecMembers(old_s, params.mecid);
    let mec_state_pre = MecState(old_s, params.mecid);
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RealmParamsSupported(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrInRange(old_s, rd, params.rtt_base, (params.rtt_num_start as int - 1) * (RMM_GRANULE_SIZE as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, params.rtt_base, (params.rtt_num_start as int) * (RMM_GRANULE_SIZE as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttConfigIsValid(old_s, params.s2sz as int, params.rtt_level_start as int, params.rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttsStateEqual(params.rtt_base, params.rtt_num_start as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!VmidsAreValid(old_s, params.vmid, params.aux_vmid) || !VmidsAreFree(old_s, params.vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (params.mecid > ImplFeatures(old_s).max_mecid ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecState(old_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiRealmParamsIsValid(old_s, params_ptr)
        && RealmParamsSupported(old_s, params)
        && !AddrInRange(old_s, rd, params.rtt_base, (params.rtt_num_start as int - 1) * (RMM_GRANULE_SIZE as int))
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegableDram(old_s, rd)
        && GranuleAt(old_s, rd).state == DELEGATED
        && AddrIsAligned(old_s, params.rtt_base, (params.rtt_num_start as int) * (RMM_GRANULE_SIZE as int))
        && RttConfigIsValid(old_s, params.s2sz as int, params.rtt_level_start as int, params.rtt_num_start as int)
        && RttsStateEqual(params.rtt_base, params.rtt_num_start as int, DELEGATED)
        && VmidsAreValid(old_s, params.vmid, params.aux_vmid)
        && VmidsAreFree(old_s, params.vmid)
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
            && realm.measurements[0] == RimInit(new_s, realm.hash_algo, params)
            && realm.rtt_level_start == params.rtt_level_start
            && realm.rtt_num_start == params.rtt_num_start
            && RealmVmidEqual(realm, params.vmid, params.aux_vmid)
            && realm.rpv == params.rpv
            && Equal(realm.feat_da, params.flags0.da)
            && Equal(realm.feat_ats, params.flags1.ats)
            && realm.ats_plane == params.ats_plane
            && Equal(realm.rtt_tree_per_plane, params.flags1.rtt_tree_per_plane)
            && realm.num_aux_planes == params.num_aux_planes
            && realm.mecid == params.mecid
            && realm.mec_policy == MecPolicy(new_s, realm.mecid)
            && (mec_state_pre == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED)
            && (mec_state_pre == MEC_STATE_SHARED ==> MecMembers(new_s, params.mecid) == mec_members_pre + 1)
            && realm.num_recs == 0
            && realm.num_vdevs == 0
            && realm.num_vsmmus == 0))
}