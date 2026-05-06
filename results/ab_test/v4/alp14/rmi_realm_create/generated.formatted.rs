pub open spec fn RMI_REALM_CREATE_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let params = RmiRealmParamsAt(old_s, params_ptr);
    let realm = RealmAt(old_s, rd);
    let mec_members_pre = MecMembers(old_s, params.mecid);
    let mec_state_pre = MecState(old_s, params.mecid);

    ((!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !RealmParamsSupported(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AddrInRange(
        rd,
        params.rtt_base,
        (params.rtt_num_start - 1) * RMM_GRANULE_SIZE,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsAligned(
        params.rtt_base,
        params.rtt_num_start * RMM_GRANULE_SIZE,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RttConfigIsValid(
        old_s,
        params.s2sz,
        params.rtt_level_start,
        params.rtt_num_start,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RttsStateEqual(
        params.rtt_base,
        params.rtt_num_start,
        DELEGATED,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!VmidsAreValid(
        old_s,
        params.vmid,
        params.aux_vmid,
    ) || !VmidsAreFree(old_s, params.vmid, params.aux_vmid)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((params.mecid as int) > (ImplFeatures().max_mecid as int) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (MecState(old_s, params.mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (result.is_Ok() ==> (GranuleAt(new_s, rd).state == RD && realm.state == REALM_NEW
        && realm.rec_index == 0 && RealmRttBaseEqual(
        old_s,
        realm,
        params.rtt_base,
        params.aux_rtt_base,
    ) && RttsStateEqual(realm.rtt_base[0], realm.rtt_num_start, RTT)
        && RttsAllProtectedEntriesState(old_s, realm.rtt_base[0], realm.rtt_num_start, UNASSIGNED)
        && RttsAllUnprotectedEntriesState(
        old_s,
        realm.rtt_base[0],
        realm.rtt_num_start,
        UNASSIGNED_NS,
    ) && RttsAllProtectedEntriesRipas(old_s, realm.rtt_base[0], realm.rtt_num_start, EMPTY)
        && realm.feat_lpa2 == params.flags0.lpa2 && realm.ipa_width == params.s2sz
        && realm.hash_algo == params.hash_algo && realm.measurements[0] == RimInit(
        old_s,
        realm.hash_algo,
        params,
    ) && realm.measurements[1] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.measurements[2]
        == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.measurements[3] == Zeros(
        RMM_REALM_MEASUREMENT_WIDTH,
    ) && realm.measurements[4] == Zeros(RMM_REALM_MEASUREMENT_WIDTH) && realm.rtt_level_start
        == params.rtt_level_start && realm.rtt_num_start == params.rtt_num_start && RealmVmidEqual(
        old_s,
        realm,
        params.vmid,
        params.aux_vmid,
    ) && realm.rpv == params.rpv && realm.feat_da == params.flags0.da && realm.feat_ats
        == params.flags1.ats && realm.ats_plane == params.ats_plane && realm.rtt_tree_per_plane
        == params.flags1.rtt_tree_per_plane && realm.num_aux_planes == params.num_aux_planes
        && realm.rtt_s2ap_encoding == params.flags1.rtt_s2ap_encoding && realm.lfa_policy
        == params.flags0.lfa_policy && realm.mecid == params.mecid && realm.mec_policy == MecPolicy(
        old_s,
        realm.mecid,
    ) && (mec_state_pre == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, params.mecid)
        == MEC_STATE_PRIVATE_ASSIGNED) && (mec_state_pre == MEC_STATE_SHARED ==> MecMembers(
        new_s,
        params.mecid,
    ) == mec_members_pre + 1) && realm.num_recs == 0 && realm.num_vdevs == 0 && realm.num_vsmmus
        == 0)))
}