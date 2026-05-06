```verus
pub open spec fn RMI_VDEV_UNMAP_spec(
    s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
) -> (result: RmiCommandReturnCode, pa: Address, top: Address)
{
    let realm = RealmAt(s, rd);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(s, walk.level));
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);

    // Failure condition checks
    if !AddrIsGranuleAligned(s, rd) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !PaIsDelegable(s, rd) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !AddrIsGranuleAligned(s, vdev_ptr) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !PaIsDelegable(s, vdev_ptr) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if GranuleAt(s, vdev_ptr).state != VDEV {
        (RMI_ERROR_INPUT, 0, 0)
    } else if vdev_pre.realm != rd {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !RttLevelIsValid(s, realm, level) || level < 2 {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !AddrIsRttLevelAligned(s, ipa, level) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !AddrIsProtected(s, ipa, realm) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if walk.level < level {
        (RMI_ERROR_RTT + walk.level, 0, walk_top)
    } else if walk.rtte.state != ASSIGNED_DEV {
        (RMI_ERROR_RTT + walk.level, 0, walk_top)
    } else if !GranulesAllVdevUnvalidated(s, walk.rtte.addr, pa_top) &&
              !GranulesAllVdevValidated(s, walk.rtte.addr, pa_top, vdev_pre) {
        (RMI_ERROR_RTT + walk.level, 0, walk_top)
    } else {
        // Success case
        // Verify success conditions implicitly:
        // - GranulesAllState(walk.rtte.addr, pa_top, DELEGATED)
        // - GranulesAllVdevUnvalidated(walk.rtte.addr, pa_top)
        // - vdev.num_map == vdev_pre.num_map - (RttLevelSize(level) << RMM_GRANULE_SIZE_ORDER)
        // - walk.rtte.state == UNASSIGNED
        // - if walk.rtte.ripas == DEV, then walk.rtte.ripas == DESTROYED
        (RMI_SUCCESS, walk.rtte.addr, walk_top)
    }
}
```

This function models the RMI_VDEV_UNMAP command according to the specification:

1. **Input parameters**: `rd` (realm descriptor), `vdev_ptr` (virtual device pointer), `ipa` (intermediate physical address), and `level` (RTT level)

2. **Context variables**: Computed from the input state including realm, vdev, RTT walk result, and derived addresses

3. **Failure conditions**: Checked in order of precedence, returning appropriate error codes:
   - Input validation (alignment, bounds, state checks)
   - RTT walk validation
   - Entry state and mapping validation

4. **Success case**: Returns `RMI_SUCCESS` with the physical address of unmapped memory and the top IPA of non-live entries

5. **Return values**: Tuple of (result code, pa, top) matching the specification output format