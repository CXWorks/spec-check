pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base, ToAddress((RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int - 1) * RMM_GRANULE_SIZE)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz as int, RmiRealmParamsAt(old_s, params_ptr).rtt_level_start as int, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttsStateEqual(RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!VmidsAreValid(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)
        || !VmidsAreFree1(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RmiRealmParamsAt(old_s, params_ptr).mecid as int) > (ImplFeatures(old_s).max_mecid as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiRealmParamsIsValid(old_s, params_ptr)
        && RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr))
        && !AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base, ToAddress((RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int - 1) * RMM_GRANULE_SIZE))
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegableDram(old_s, rd)
        && GranuleAt(old_s, rd).state == DELEGATED
        && AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int * RMM_GRANULE_SIZE)
        && RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz as int, RmiRealmParamsAt(old_s, params_ptr).rtt_level_start as int, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int)
        && RttsStateEqual(RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int, DELEGATED)
        && VmidsAreValid(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)
        && VmidsAreFree1(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)
        && (RmiRealmParamsAt(old_s, params_ptr).mecid as int) <= (ImplFeatures(old_s).max_mecid as int)
        && MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) != MEC_STATE_PRIVATE_ASSIGNED)
        ==> (result.is_Ok()
            && GranuleAt(new_s, rd).state == RD
            && RealmAt(new_s, rd).state == REALM_NEW
            && RealmAt(new_s, rd).rec_index == 0
            && RealmRttBaseEqual(RealmAt(new_s, rd), RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).aux_rtt_base)
            && RttsStateEqual(RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, RTT)
            && RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, UNASSIGNED)
            && RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, UNASSIGNED_NS)
            && RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, EMPTY)
            && RealmAt(new_s, rd).feat_lpa2 == RmiRealmParamsAt(old_s, params_ptr).flags0.lpa2
            && RealmAt(new_s, rd).ipa_width == RmiRealmParamsAt(old_s, params_ptr).s2sz
            && RealmAt(new_s, rd).hash_algo == RmiRealmParamsAt(old_s, params_ptr).hash_algo
            && RealmAt(new_s, rd).measurements[0] == RimInit(new_s, RealmAt(new_s, rd).hash_algo, RmiRealmParamsAt(old_s, params_ptr))
            && RealmAt(new_s, rd).rtt_level_start == RmiRealmParamsAt(old_s, params_ptr).rtt_level_start
            && RealmAt(new_s, rd).rtt_num_start == RmiRealmParamsAt(old_s, params_ptr).rtt_num_start
            && RealmVmidEqual(RealmAt(new_s, rd), RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)
            && RealmAt(new_s, rd).rpv == RmiRealmParamsAt(old_s, params_ptr).rpv
            && RealmAt(new_s, rd).feat_da == RmiRealmParamsAt(old_s, params_ptr).flags0.da
            && RealmAt(new_s, rd).feat_ats == RmiRealmParamsAt(old_s, params_ptr).flags1.ats
            && RealmAt(new_s, rd).ats_plane == RmiRealmParamsAt(old_s, params_ptr).ats_plane
            && RealmAt(new_s, rd).rtt_tree_per_plane == RmiRealmParamsAt(old_s, params_ptr).flags1.rtt_tree_per_plane
            && RealmAt(new_s, rd).num_aux_planes == RmiRealmParamsAt(old_s, params_ptr).num_aux_planes
            && RealmAt(new_s, rd).rtt_s2ap_encoding == RmiRealmParamsAt(old_s, params_ptr).flags1.rtt_s2ap_encoding
            && RealmAt(new_s, rd).lfa_policy == RmiRealmParamsAt(old_s, params_ptr).flags0.lfa_policy
            && RealmAt(new_s, rd).mecid == RmiRealmParamsAt(old_s, params_ptr).mecid
            && RealmAt(new_s, rd).mec_policy == MecPolicy(new_s, RealmAt(new_s, rd).mecid)
            && (MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_UNASSIGNED
                ==> MecState(new_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED)
            && (MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_SHARED
                ==> (MecMembers(new_s, RmiRealmParamsAt(old_s, params_ptr).mecid) as int) == (MecMembers(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) as int) + 1)
            && RealmAt(new_s, rd).num_recs == 0
            && RealmAt(new_s, rd).num_vdevs == 0
            && RealmAt(new_s, rd).num_vsmmus == 0))
}