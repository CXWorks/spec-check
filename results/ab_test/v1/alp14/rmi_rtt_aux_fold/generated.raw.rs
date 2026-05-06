```rust
pub open spec fn RMI_RTT_AUX_FOLD_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
) -> (result: Result<Address, RmiStatusCode>)
    // Failure condition: rd_align
    requires AddrIsGranuleAligned(rd)
    // Failure condition: rd_bound
    requires PaIsDelegable(rd)
    // Failure condition: rd_state
    requires GranuleAt(s, rd).state == RD
    // Failure condition: level_bound
    requires RttLevelIsValid(s, RealmAt(s, rd), level) && !RttLevelIsStarting(s, RealmAt(s, rd), level)
    // Failure condition: ipa_align
    requires AddrIsRttLevelAligned(ipa, level - 1)
    // Failure condition: ipa_bound
    requires AddrIsProtected(s, ipa, RealmAt(s, rd))
    // Failure condition: index_bound
    requires (RealmAt(s, rd).rtt_tree_per_plane == FEATURE_TRUE
              && index != RMM_RTT_TREE_PRIMARY
              && index <= RealmAt(s, rd).num_aux_planes)
    // Derive context values
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let fold_pre = RttFold(s, RttAt(s, walk.rtte.addr));
    // Failure condition: rtt_walk
    requires walk.level == level - 1
    // Failure condition: rtte_state
    requires walk.rtte.state == TABLE
    // Failure condition: rtt_homo
    requires RttIsHomogeneous(s, RttAt(s, walk.rtte.addr))
    // Success conditions
    ensures {
        // rtte_state: walk.rtte.state == fold_pre.state
        result.is_Ok() && fold_pre.state == walk.rtte.state
    }
    // On success, return the RTT address
    ensures result.is_Ok() ==> result.get_Ok_0() == walk.rtte.addr
```