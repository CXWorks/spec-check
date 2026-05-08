```rust
pub open spec fn RMI_RTT_SET_RIPAS_spec(
    s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
) -> (result: Result<Address, RmiStatusCode>)
{
    // Context setup
    let realm = RealmAt(s, rd);
    let realm_pre = RealmAt(s, rd);
    let rec = RecAt(s, rec_ptr);
    let walk = RttWalk(s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        s,
        RttAt(s, walk.rtt_addr),
        walk.level,
        base,
        top,
        (rec.ripas_value == RAM) && (rec.ripas_destroyed != CHANGE_DESTROYED),
    );

    // Failure condition checks (in order of precedence)
    if !AddrIsGranuleAligned(rd) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rd) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RD {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(rec_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rec_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, rec_ptr).state != REC {
        Err(RMI_ERROR_INPUT)
    } else if rec.state == REC_RUNNING {
        Err(RMI_ERROR_REC)
    } else if rec.owner != rd {
        Err(RMI_ERROR_REC)
    } else if UInt(top) <= UInt(base) {
        Err(RMI_ERROR_INPUT)
    } else if base != rec.ripas_addr {
        Err(RMI_ERROR_INPUT)
    } else if UInt(top) > UInt(rec.ripas_top) {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(base, walk.level) && ripas_pre != rec.ripas_value {
        Err(RMI_ERROR_RTT)
    } else if !AddrIsGranuleAligned(top) {
        Err(RMI_ERROR_INPUT)
    } else if UInt(base) == UInt(walk_top_pre) && ripas_pre != rec.ripas_value {
        Err(RMI_ERROR_RTT)
    } else if AddrRangeIsAuxLive(s, base, top, realm_pre) {
        Err(RMI_ERROR_RTT)
    } else {
        // Success condition: return MinAddress(top, walk_top_pre)
        let out_top = MinAddress(top, walk_top_pre);
        Ok(out_top)
    }
}
```

**Key aspects of the specification:**

1. **Context Setup**: Establishes realm, rec, RTT walk result, and pre-condition values
2. **Failure Conditions**: Checks are ordered according to the specified precedence constraints:
   - Input validation (alignment, bounds, state checks)
   - REC validation (state, ownership)
   - Size and address range validation
   - RTT and auxiliary live checks
3. **Success Condition**: Returns `Ok(out_top)` where `out_top = MinAddress(top, walk_top_pre)`, which represents the top IPA of the range whose RIPAS was modified
4. **Implicit Success Conditions**: The spec assumes that on success:
   - RTT entries in the modified range have the correct RIPAS value
   - `rec.ripas_addr` is updated to `MinAddress(top, walk_top_pre)`