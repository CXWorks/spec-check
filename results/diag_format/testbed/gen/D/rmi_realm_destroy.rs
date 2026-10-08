pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM))
    && (result.is_Ok() ==> RttsGranuleState(RealmAt(old_s, rd).rtt_base[0], RealmAt(old_s, rd).rtt_num_start) == DELEGATED)
    && (result.is_Ok() ==> GranuleAt(new_s, rd).state == DELEGATED)
    && (result.is_Ok() ==> VmidsAreValid(old_s, RealmAt(old_s, rd).vmid[0], RealmAt(old_s, rd).aux_vmid))
    && (result.is_Ok() ==> (RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE ==> MecState(old_s, RealmAt(old_s, rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED))
    && (result.is_Ok() ==> (RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED ==> MecMembers(old_s, RealmAt(old_s, rd).mecid) == MecMembers(old_s, RealmAt(old_s, rd).mecid) - 1))
}