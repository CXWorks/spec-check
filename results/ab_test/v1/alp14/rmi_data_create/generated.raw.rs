```rust
pub open spec fn RMI_DATA_CREATE_spec(
    s: S,
    rd: Address,
    data: Address,
    ipa: Address,
    src: Address,
    flags: RmiDataFlags,
) -> (result: Result<(), RmiStatusCode>, s_post: S)
    ensures
        // Input validation - alignment checks
        !AddrIsGranuleAligned(src) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        !AddrIsGranuleAligned(data) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        !AddrIsGranuleAligned(rd) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        !AddrIsGranuleAligned(ipa) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        
        // Input validation - PAS and delegation checks
        !GranuleAccessPermitted(src, PAS_NS) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        !PaIsDelegableDram(data) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        !PaIsDelegable(rd) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        
        // Input validation - granule state checks
        GranuleAt(s, data).state != DELEGATED ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        GranuleAt(s, rd).state != RD ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        
        // Input validation - address bound checks
        !AddrIsProtected(ipa, RealmAt(s, rd)) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        let realm_pre = RealmAt(s, rd);
        (realm_pre.feat_lpa2 == FEATURE_FALSE && UInt(data) >= (1 << 48)) ==> 
            result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT,
        
        // Realm state validation
        let realm_pre = RealmAt(s, rd);
        realm_pre.state != REALM_NEW ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_REALM,
        
        // RTT walk and entry validation
        let realm_pre = RealmAt(s, rd);
        let walk = RttWalk(s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
        walk.level < RMM_RTT_PAGE_LEVEL ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT,
        walk.rtte.state != UNASSIGNED ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT,
        
        // Success conditions
        result.is_Ok() ==> {
            let realm_pre = RealmAt(s, rd);
            let walk = RttWalk(s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
            let entry_idx = RttEntryIndex(s, ipa, walk.level);
            let realm_post = RealmAt(s_post, rd);
            
            // Data granule state transition
            GranuleAt(s_post, data).state == DATA &&
            
            // Data content copied
            ContentsEqual(GranuleAt(s_post, data), GranuleAt(s, src)) &&
            
            // RTT entry state transitions
            RttEntryAt(s_post, RttAt(s_post, walk.rtt_addr), entry_idx).state == ASSIGNED &&
            RttEntryAt(s_post, RttAt(s_post, walk.rtt_addr), entry_idx).ripas == RAM &&
            RttEntryAt(s_post, RttAt(s_post, walk.rtt_addr), entry_idx).addr == data &&
            RttEntryAt(s_post, RttAt(s_post, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_CACHEABLE &&
            RttEntryAt(s_post, RttAt(s_post, walk.rtt_addr), entry_idx).sh == SHAREABILITY_INNER &&
            
            // RIM extended with measurement data
            realm_post.measurements[0] == RimExtendData(s, realm_pre, ipa, data, flags)
        }
;
```