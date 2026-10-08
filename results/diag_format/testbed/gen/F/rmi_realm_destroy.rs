pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM))
    && (result.is_Ok() ==> (
        GranuleAt(new_s, rd).state == DELEGATED
        && RttsGranuleState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int) == DELEGATED
        && VmidsAreFree(new_s, RealmAt(new_s, rd).vmid)
        && (MecPolicy(new_s, MecMembers(new_s, RealmAt(new_s, rd).mecid)) == MEC_POLICY_PRIVATE ==> MecState(new_s, MecMembers(new_s, RealmAt(new_s, rd).mecid) as u64) == MEC_STATE_PRIVATE_UNASSIGNED)
        && (MecPolicy(new_s, MecMembers(new_s, RealmAt(new_s, rd).mecid)) == MEC_POLICY_SHARED ==> MecState(new_s, MecMembers(new_s, RealmAt(new_s, rd).mecid) as u64) == MEC_STATE_SHARED)
    ))
}