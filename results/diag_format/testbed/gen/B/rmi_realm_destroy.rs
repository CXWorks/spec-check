pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM))
    && (result.is_Ok() ==> (RttsStateEqual(RealmRttBase(old_s, rd, 0), old_s.RealmAt(rd).rtt_num_start, DELEGATED) && GranuleAt(new_s, rd).state == DELEGATED && VmidsAreFree(old_s.RealmAt(rd).vmid) && (old_s.RealmAt(rd).mec_policy == MEC_POLICY_PRIVATE ==> MecState(old_s.RealmAt(rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED) && (old_s.RealmAt(rd).mec_policy == MEC_POLICY_SHARED ==> MecMembers(old_s.RealmAt(rd).mecid) == old_s.MecMembers(old_s.RealmAt(rd).mecid) - 1)))
}