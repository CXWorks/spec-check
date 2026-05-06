```rust
pub open spec fn RMI_RTT_DESTROY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
) -> (result: RmiCommandReturnCode, rtt: Address, top: Address) {
    let realm = RealmAt(rd);
    let walk = RttWalk(s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions (in order of precedence)
    if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level) {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if !AddrIsRttLevelAligned(ipa, level - 1) {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if UInt(ipa) >= (1 << realm.ipa_width) {
        (RMI_ERROR_INPUT, 0 as Address, 0 as Address)
    } else if walk.level < level - 1 {
        (combine_rmi_error_rtt(RMI_ERROR_RTT, walk.level), 0 as Address, walk_top)
    } else if walk.rtte.state != TABLE {
        (combine_rmi_error_rtt(RMI_ERROR_RTT, walk.level), 0 as Address, walk_top)
    } else if RttIsLive(s, RttAt(s, walk.rtte.addr)) {
        (combine_rmi_error_rtt(RMI_ERROR_RTT, level), 0 as Address, ipa)
    } else if AddrIsAuxRef(ipa, realm) {
        (combine_rmi_error_rtt(RMI_ERROR_RTT, walk.level), 0 as Address, 0 as Address)
    } else {
        // Success condition
        (RMI_SUCCESS, walk.rtte.addr, walk_top)
    }
}
```

**Key aspects of the specification:**

1. **Context variables** are computed as specified in B4.3.39.1.2
2. **Failure conditions** are checked in the order defined by the failure condition ordering (B4.3.39.2.1)
3. **Return values**:
   - `result`: RmiCommandReturnCode (success or specific error code with level)
   - `rtt`: Address of destroyed RTT (valid only on RMI_SUCCESS)
   - `top`: Top IPA of non-live entries
4. **Success case** returns RMI_SUCCESS with the RTT address and walk_top value
5. Uses helper function `combine_rmi_error_rtt()` to encode error level into the return code as specified in the RMI specification