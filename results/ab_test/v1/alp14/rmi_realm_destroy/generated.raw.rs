```verus
pub open spec fn RMI_REALM_DESTROY_spec(
    s: S,
    rd: Address,
) -> (result: Result<(), RmiStatusCode>)
    requires
        AddrIsGranuleAligned(rd),
        PaIsDelegable(rd),
        GranuleAt(s, rd).state == RmmGranuleState::RD,
        !RealmIsLive(s, rd),
    ensures
        result.is_Ok(),
        {
            let realm_pre = RealmAt(s, rd);
            let mec_members_pre = MecMembers(s, realm_pre.mecid);
            
            // rtt_state: All RTT entries become DELEGATED
            RttsStateEqual(realm_pre.rtt_base[0], realm_pre.rtt_num_start, RmmGranuleState::DELEGATED) &&
            
            // rd_state: RD granule becomes DELEGATED
            GranuleAt(s, rd).state == RmmGranuleState::DELEGATED &&
            
            // vmid: VMIDs are freed
            VmidsAreFree(s, realm_pre.vmid) &&
            
            // mecid_private: If MEC policy is PRIVATE, MEC state becomes PRIVATE_UNASSIGNED
            (realm_pre.mec_policy == RmmMecPolicy::MEC_POLICY_PRIVATE ==>
                MecState(s, realm_pre.mecid) == RmmMecState::MEC_STATE_PRIVATE_UNASSIGNED) &&
            
            // mec_members: If MEC policy is SHARED, MEC member count decrements
            (realm_pre.mec_policy == RmmMecPolicy::MEC_POLICY_SHARED ==>
                MecMembers(s, realm_pre.mecid) == mec_members_pre - 1)
        }
```

**Failure condition variants:**

```verus
pub open spec fn RMI_REALM_DESTROY_spec_fail_rd_align(
    s: S,
    rd: Address,
) -> (result: Result<(), RmiStatusCode>)
    requires !AddrIsGranuleAligned(rd),
    ensures ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT),

pub open spec fn RMI_REALM_DESTROY_spec_fail_rd_bound(
    s: S,
    rd: Address,
) -> (result: Result<(), RmiStatusCode>)
    requires !PaIsDelegable(rd),
    ensures ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT),

pub open spec fn RMI_REALM_DESTROY_spec_fail_rd_state(
    s: S,
    rd: Address,
) -> (result: Result<(), RmiStatusCode>)
    requires GranuleAt(s, rd).state != RmmGranuleState::RD,
    ensures ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT),

pub open spec fn RMI_REALM_DESTROY_spec_fail_realm_live(
    s: S,
    rd: Address,
) -> (result: Result<(), RmiStatusCode>)
    requires RealmIsLive(s, rd),
    ensures ResultEqual(result, RmiStatusCode::RMI_ERROR_REALM),
```