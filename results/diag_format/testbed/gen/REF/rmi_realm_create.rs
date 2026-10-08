pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RealmParamsSupported(old_s, RealmAt(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrInRange(old_s, rd, RealmAt(old_s, rd).rtt_base[0], (RealmAt(old_s, rd).rtt_num_start - 1) * RMM_GRANULE_SIZE as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, RealmAt(old_s, params_ptr).rtt_base[0], RealmAt(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttConfigIsValid(old_s, RealmAt(old_s, params_ptr).s2sz as int, RealmAt(old_s, params_ptr).rtt_level_start as int, RealmAt(old_s, params_ptr).rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttsStateEqual(old_s, RealmAt(old_s, params_ptr).rtt_base[0], RealmAt(old_s, params_ptr).rtt_num_start, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!VmidsAreValid(old_s, RealmAt(old_s, params_ptr).vmid[0], RealmAt(old_s, params_ptr).aux_vmid[0..3]) || !VmidsAreFree(old_s, RealmAt(old_s, params_ptr).vmid[0], RealmAt(old_s, params_ptr).aux_vmid[0..3])) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, params_ptr).mecid) > (ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, RealmAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == RD)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rec_index == 0)
  && (result.is_Ok() ==> RealmRttBaseEqual(RealmAt(new_s, rd), params_ptr, RealmAt(new_s, params_ptr).aux_rtt_base))
  && (result.is_Ok() ==> RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, RTT))
  && (result.is_Ok() ==> RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED))
  && (result.is_Ok() ==> RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, UNASSIGNED_NS))
  && (result.is_Ok() ==> RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, EMPTY))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_lpa2, RealmAt(new_s, params_ptr).flags0.lpa2))
  && (result.is_Ok() ==> RealmAt(new_s, rd).ipa_width == RealmAt(new_s, params_ptr).s2sz)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).hash_algo, RealmAt(new_s, params_ptr).hash_algo))
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimInit(new_s, RealmAt(new_s, rd).hash_algo, RealmAt(new_s, params_ptr)))
  && (result.is_Ok() ==> (RealmAt(new_s, rd).measurements[1] == 0 && RealmAt(new_s, rd).measurements[2] == 0 && RealmAt(new_s, rd).measurements[3] == 0 && RealmAt(new_s, rd).measurements[4] == 0))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(new_s, params_ptr).rtt_level_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(new_s, params_ptr).rtt_num_start)
  && (result.is_Ok() ==> RealmVmidEqual(RealmAt(new_s, rd), RealmAt(new_s, params_ptr).vmid[0], RealmAt(new_s, params_ptr).aux_vmid))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rpv == RealmAt(new_s, params_ptr).rpv)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_da, RealmAt(new_s, params_ptr).flags0.da))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_ats, RealmAt(new_s, params_ptr).flags1.ats))
  && (result.is_Ok() ==> RealmAt(new_s, rd).ats_plane == RealmAt(new_s, params_ptr).ats_plane)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_tree_per_plane, RealmAt(new_s, params_ptr).flags1.rtt_tree_per_plane))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_aux_planes == RealmAt(new_s, params_ptr).num_aux_planes)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_s2ap_encoding, RealmAt(new_s, params_ptr).flags1.rtt_s2ap_encoding))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).lfa_policy, RealmAt(new_s, params_ptr).flags0.lfa_policy))
  && (result.is_Ok() ==> RealmAt(new_s, rd).mecid == RealmAt(new_s, params_ptr).mecid)
  && (result.is_Ok() ==> RealmAt(new_s, rd).mec_policy == MecPolicy(new_s, RealmAt(new_s, rd).mecid))
  && (result.is_Ok() && MecState(old_s, RealmAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, RealmAt(new_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED)
  && (result.is_Ok() && MecState(old_s, RealmAt(old_s, params_ptr).mecid) == MEC_STATE_SHARED ==> MecMembers(new_s, RealmAt(new_s, params_ptr).mecid) == MecMembers(old_s, RealmAt(old_s, params_ptr).mecid) + 1)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_recs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == 0)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiRealmParamsIsValid(old_s, params_ptr) &&
       RealmParamsSupported(old_s, RealmAt(old_s, params_ptr)) &&
       !(AddrInRange(old_s, rd, RealmAt(old_s, rd).rtt_base[0], (RealmAt(old_s, rd).rtt_num_start - 1) * RMM_GRANULE_SIZE as int)) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegableDram(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != DELEGATED) &&
       AddrIsAligned(old_s, RealmAt(old_s, params_ptr).rtt_base[0], RealmAt(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE as int) &&
       RttConfigIsValid(old_s, RealmAt(old_s, params_ptr).s2sz as int, RealmAt(old_s, params_ptr).rtt_level_start as int, RealmAt(old_s, params_ptr).rtt_num_start as int) &&
       RttsStateEqual(old_s, RealmAt(old_s, params_ptr).rtt_base[0], RealmAt(old_s, params_ptr).rtt_num_start, DELEGATED) &&
       !((VmidsAreValid(old_s, RealmAt(old_s, params_ptr).vmid[0], RealmAt(old_s, params_ptr).aux_vmid[0..3]) || VmidsAreFree(old_s, RealmAt(old_s, params_ptr).vmid[0], RealmAt(old_s, params_ptr).aux_vmid[0..3])) &&
       !((RealmAt(old_s, params_ptr).mecid) > (ImplFeatures(old_s).max_mecid)) &&
       !(MecState(old_s, RealmAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(old_s, rd).rtt_level_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_recs == RealmAt(old_s, rd).num_recs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
}