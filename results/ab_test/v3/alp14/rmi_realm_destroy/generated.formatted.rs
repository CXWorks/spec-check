pub open spec fn RMI_REALM_DESTROY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let mec_members_pre = MecMembers(old_s, realm_pre.mecid);

    // Failure condition: rd_align
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: realm_live (checked after rd_bound and rd_state)
    (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM))
        &&
    // Success conditions (when result is Ok)
    (result.is_Ok() ==> (
    // rtt_state: All RTT granules become DELEGATED
    RttsStateEqual(old_s, realm_pre.rtt_base, realm_pre.rtt_num_start, DELEGATED)
        &&
    // rd_state: RD granule becomes DELEGATED
    GranuleAt(new_s, rd).state == DELEGATED &&
    // vmid: VMIDs are freed
    VmidsAreFree(new_s, realm_pre.vmid)
        &&
    // mecid_private: If private MEC policy, MEC state becomes PRIVATE_UNASSIGNED
    (realm_pre.mec_policy == MEC_POLICY_PRIVATE ==> MecState(new_s, realm_pre.mecid)
        == MEC_STATE_PRIVATE_UNASSIGNED)
        &&
    // mec_members: If shared MEC policy, member count decreases by 1
    (realm_pre.mec_policy == MEC_POLICY_SHARED ==> MecMembers(new_s, realm_pre.mecid)
        == mec_members_pre - 1)))
}