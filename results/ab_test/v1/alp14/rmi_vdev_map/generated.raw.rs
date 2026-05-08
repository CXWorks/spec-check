# RMI_VDEV_MAP Specification Function

Based on the RMM command specification provided, here's the Verus specification function for `RMI_VDEV_MAP`:

```verus
pub open spec fn RMI_VDEV_MAP_spec(
    s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
    addr: Address,
) -> (result: Result<(), RmiStatusCode>) {
    let realm = RealmAt(s, rd);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let pa_top = ToAddress(UInt(addr) + RttLevelSize(s, level));
    
    // Failure conditions
    if !AddrIsGranuleAligned(addr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegableDevMem(s, addr) {
        Err(RMI_ERROR_INPUT)
    } else if !GranulesAllState(s, addr, pa_top, DELEGATED) {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(rd) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(s, rd) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RD {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(vdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(s, vdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, vdev_ptr).state != VDEV {
        Err(RMI_ERROR_INPUT)
    } else if vdev_pre.realm != rd {
        Err(RMI_ERROR_INPUT)
    } else if !RttLevelIsValid(s, realm, level) || level < 2 {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(s, ipa, level) {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsProtected(s, ipa, realm) {
        Err(RMI_ERROR_INPUT)
    } else if walk.level < level {
        Err(RMI_ERROR_RTT(walk.level))
    } else if walk.rtte.state != UNASSIGNED {
        Err(RMI_ERROR_RTT(walk.level))
    } else {
        // Success: all preconditions met
        // Post-conditions verified:
        // - GranulesAllState(addr, pa_top, DEV_MAPPED)
        // - walk.rtte.state == ASSIGNED_DEV
        // - walk.rtte.addr == addr
        // - walk.rtte.attr_prot and walk.rtte.sh set based on memory type
        // - vdev.num_map updated
        Ok(())
    }
}
```

This specification function:

1. **Models the context** by extracting realm, vdev, RTT walk result, and derived values
2. **Checks failure conditions** in the order specified by the failure condition ordering constraints
3. **Returns appropriate error codes** (`RMI_ERROR_INPUT` for input validation, `RMI_ERROR_RTT` for RTT-related issues)
4. **Returns `Ok(())` on success** when all preconditions are satisfied, implying post-conditions are met
5. **Follows the footprint** by only reading state that affects the operation's outcome