```verus
pub open spec fn RMI_REALM_DESTROY_spec(s: S, rd: Address, result: RmiCommandReturnCode) -> bool {
    let realm_pre = RealmAt(s, rd);
    let mec_members_pre = MecMembers(s, realm_pre.mecid);
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(s, rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let realm_live_fail = RealmIsLive(s, rd) && ResultEqual(result, RMI_ERROR_REALM);
    
    // Success conditions
    let rtt_state_success = RttsStateEqual(realm_pre.rtt_base[0], realm_pre.rtt_num_start, DELEGATED);
    let rd_state_success = GranuleAt(s, rd).state == DELEGATED;
    let vmid_success = VmidsAreFree(s, realm_pre.vmid);
    let mecid_private_success = realm_pre.mec_policy == MEC_POLICY_PRIVATE ==> MecState(s, realm_pre.mecid) == MEC_STATE_PRIVATE_UNASSIGNED;
    let mec_members_success = realm_pre.mec_policy == MEC_POLICY_SHARED ==> MecMembers(s, realm_pre.mecid) == mec_members_pre - 1;
    
    // Check preconditions
    let rd_aligned = AddrIsGranuleAligned(s, rd);
    let rd_delegable = PaIsDelegable(s, rd);
    let rd_is_rd = GranuleAt(s, rd).state == RD;
    let realm_not_live = !RealmIsLive(s, rd);
    
    // Either we fail with one of the failure conditions, or we succeed with all success conditions
    (rd_align_fail || rd_bound_fail || rd_state_fail || realm_live_fail) ||
    (rd_aligned && rd_delegable && rd_is_rd && realm_not_live &&
     result.is_Ok() && rtt_state_success && rd_state_success && vmid_success &&
     mecid_private_success && mec_members_success)
}
```