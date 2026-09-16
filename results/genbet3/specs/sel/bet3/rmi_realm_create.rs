pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RmmGlobal(old_s).dynamic.pat_valid != RMM_TRUE ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base,(RmiRealmParamsAt(old_s, params_ptr).rtt_num_start - 1) * RmmGlobal(old_s).dynamic.rmi_granule_size) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsPopulatedConventional(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTrackedFine(old_s, rd) ==> (ResultEqual(result, RMI_ERROR_TRACKING) && ResultEqual(result, TrackingToRmiResult(old_s, rd))))
  && (GranuleAt(old_s, rd).state != GRAN_DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start * RmmGlobal(old_s).dynamic.rmi_granule_size) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz,RmiRealmParamsAt(old_s, params_ptr).rtt_level_start,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttsStateEqual(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start,GRAN_DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiRealmParamsAt(old_s, params_ptr).ats_plane > RmiRealmParamsAt(old_s, params_ptr).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VmidsAvailable(old_s, RmiRealmParamsAt(old_s, params_ptr).num_aux_planes + 1) ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (!MecidAvailable(old_s, RmiRealmParamsAt(old_s, params_ptr).flags0.mec_policy) ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (result.is_Ok() ==> RmmGlobal(new_s).dynamic.num_realms == RmmGlobal(new_s).dynamic.num_realms + 1)
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == GRAN_RD)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW)
  && (result.is_Ok() ==> RealmRttBaseEqual(new_s, RealmAt(new_s, rd), RmiRealmParamsAt(new_s, params_ptr).rtt_base, RmiRealmParamsAt(new_s, params_ptr).aux_rtt_base))
  && (result.is_Ok() ==> RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start,GRAN_RTT))
  && (result.is_Ok() ==> RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start,RTTE_VOID))
  && (result.is_Ok() ==> RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start,RTTE_UNMAPPED_NS))
  && (result.is_Ok() ==> RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start,RIPAS_EMPTY))
  && (result.is_Ok() ==> RealmAt(new_s, rd).ipa_width == RmiRealmParamsAt(new_s, params_ptr).s2sz)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).hash_algo, RmiRealmParamsAt(new_s, params_ptr).hash_algo))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_level_start == RmiRealmParamsAt(new_s, params_ptr).rtt_level_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_num_start == RmiRealmParamsAt(new_s, params_ptr).rtt_num_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rpv == RmiRealmParamsAt(new_s, params_ptr).rpv)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_da, RmiRealmParamsAt(new_s, params_ptr).flags0.da))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_ats, RmiRealmParamsAt(new_s, params_ptr).flags1.ats))
  && (result.is_Ok() ==> RealmAt(new_s, rd).ats_plane == RmiRealmParamsAt(new_s, params_ptr).ats_plane)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_tree_per_plane, RmiRealmParamsAt(new_s, params_ptr).flags1.rtt_tree_per_plane))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_aux_planes == RmiRealmParamsAt(new_s, params_ptr).num_aux_planes)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_s2ap_encoding, RmiRealmParamsAt(new_s, params_ptr).flags1.rtt_s2ap_encoding))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).lfa_policy, RmiRealmParamsAt(new_s, params_ptr).flags0.lfa_policy))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).mec_policy, RmiRealmParamsAt(new_s, params_ptr).flags0.mec_policy))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_recs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == 0)
  && ((!(RmmGlobal(old_s).dynamic.pat_valid != RMM_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiRealmParamsIsValid(old_s, params_ptr) &&
       RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr)) &&
       !(AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base,(RmiRealmParamsAt(old_s, params_ptr).rtt_num_start - 1) * RmmGlobal(old_s).dynamic.rmi_granule_size)) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsPopulatedConventional(old_s, rd) &&
       PaIsTrackedFine(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_DELEGATED) &&
       AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start * RmmGlobal(old_s).dynamic.rmi_granule_size) &&
       RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz,RmiRealmParamsAt(old_s, params_ptr).rtt_level_start,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start) &&
       RttsStateEqual(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start,GRAN_DELEGATED) &&
       !(RmiRealmParamsAt(old_s, params_ptr).ats_plane > RmiRealmParamsAt(old_s, params_ptr).num_aux_planes) &&
       VmidsAvailable(old_s, RmiRealmParamsAt(old_s, params_ptr).num_aux_planes + 1) &&
       MecidAvailable(old_s, RmiRealmParamsAt(old_s, params_ptr).flags0.mec_policy))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ipa_width == RealmAt(old_s, rd).ipa_width)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(old_s, rd).rtt_level_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rpv == RealmAt(old_s, rd).rpv)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ats_plane == RealmAt(old_s, rd).ats_plane)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_aux_planes == RealmAt(old_s, rd).num_aux_planes)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_recs == RealmAt(old_s, rd).num_recs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
  && (!(result.is_Ok() && (GranuleAt(new_s, rd).state == GRAN_RD)) ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
}