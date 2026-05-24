pub open spec fn rmi_realm_destroy_spec(result: RmiCommandReturnCode, rd: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let mec_members_pre = MecMembers(old_s, realm_pre.mecid);
    
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM))
    && (
      (AddrIsGranuleAligned(old_s, rd)
       && PaIsDelegable(old_s, rd)
       && GranuleAt(old_s, rd).state == RD
       && !RealmIsLive(old_s, rd))
      ==> (
        result.is_Ok()
        && RttsStateEqual(realm_pre.rtt_base, realm_pre.rtt_num_start, DELEGATED)
        && GranuleAt(new_s, rd).state == DELEGATED
        && VmidsAreFree(new_s, realm_pre.vmid)
        && (realm_pre.mec_policy == MEC_POLICY_PRIVATE
            ==> MecState(new_s, realm_pre.mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
        && (realm_pre.mec_policy == MEC_POLICY_SHARED
            ==> MecMembers(new_s, realm_pre.mecid) == mec_members_pre - 1)
      )
    )
}