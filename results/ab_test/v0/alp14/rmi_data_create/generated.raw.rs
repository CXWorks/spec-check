```verus
pub open spec fn RMI_DATA_CREATE_spec(s: S, rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags) -> bool {
    let realm_pre = RealmAt(s, rd);
    let walk = RttWalk(s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let realm = RealmAt(s, rd);
    
    // Failure conditions
    (AddrIsGranuleAligned(src) &&
     GranuleAccessPermitted(s, src, PAS_NS) &&
     AddrIsGranuleAligned(data) &&
     PaIsDelegableDram(s, data) &&
     GranuleAt(s, data).state == DELEGATED &&
     (realm_pre.feat_lpa2 == FEATURE_TRUE || UInt(data) < pow(2, 48)) &&
     AddrIsGranuleAligned(rd) &&
     PaIsDelegable(s, rd) &&
     GranuleAt(s, rd).state == RD &&
     AddrIsGranuleAligned(ipa) &&
     AddrIsProtected(s, ipa, realm_pre) &&
     realm_pre.state == REALM_NEW &&
     walk.level == RMM_RTT_PAGE_LEVEL &&
     walk.rtte.state == UNASSIGNED) ==>
    
    // Success conditions
    (GranuleAt(s, data).state == DATA &&
     walk.rtte.state == ASSIGNED &&
     walk.rtte.ripas == RAM &&
     walk.rtte.addr == data &&
     walk.rtte.attr_prot == MEMATTR_CACHEABLE &&
     walk.rtte.sh == SHAREABILITY_INNER &&
     realm.measurements[0] == RimExtendData(s, realm_pre, ipa, data, flags))
}
```