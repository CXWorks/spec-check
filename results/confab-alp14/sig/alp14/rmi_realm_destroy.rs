pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM))
    && ((AddrIsGranuleAligned(old_s, rd)
         && PaIsDelegable(old_s, rd)
         && GranuleAt(old_s, rd).state == RD
         && !RealmIsLive(old_s, rd))
        ==> (result.is_Ok()
             && RttsStateEqual(RealmAt(old_s, rd).rtt_base[0], RealmAt(old_s, rd).rtt_num_start as int, DELEGATED)
             && GranuleAt(new_s, rd).state == DELEGATED
             && VmidsAreFree(new_s, RealmAt(old_s, rd).vmid)
             && (RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE
                 ==> MecState(new_s, RealmAt(old_s, rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
             && (RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED
                 ==> MecMembers(new_s, RealmAt(old_s, rd).mecid) == MecMembers(old_s, RealmAt(old_s, rd).mecid) - 1)))
}