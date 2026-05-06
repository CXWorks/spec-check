```rust
pub open spec fn RMI_VDEV_VALIDATE_MAPPING_spec(
    s: S,
    rd: Address,
    rec_ptr: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    base: Address,
    top: Address,
) -> (result: Result<Address, RmiStatusCode>, s_post: S)
{
    let realm = RealmAt(s, rd);
    let realm_pre = realm;
    let rec = RecAt(s, rec_ptr);
    let pdev = PdevAt(s, pdev_ptr);
    let vdev = VdevAt(s, vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(
        RttAt(s, walk.rtt_addr),
        walk.level,
        base,
        top,
        false,
    );

    // Failure conditions with early returns
    if !AddrIsGranuleAligned(s, rd) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(s, rd) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rd).state != RmmGranuleState::RD {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(s, rec_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(s, rec_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rec_ptr).state != RmmGranuleState::REC {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if rec.state == RmmRecState::REC_RUNNING {
        (Err(RmiStatusCode::RMI_ERROR_REC), s)
    } else if rec.owner != rd {
        (Err(RmiStatusCode::RMI_ERROR_REC), s)
    } else if !AddrIsGranuleAligned(s, pdev_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(s, pdev_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, pdev_ptr).state != RmmGranuleState::PDEV {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(s, vdev_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(s, vdev_ptr) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, vdev_ptr).state != RmmGranuleState::VDEV {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if vdev.pdev != pdev_ptr {
        (Err(RmiStatusCode::RMI_ERROR_DEVICE), s)
    } else if UInt(top) <= UInt(base) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if base != rec.dev_mem_addr {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if UInt(top) > UInt(rec.dev_mem_top) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if !AddrIsRttLevelAligned(s, base, walk.level) {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if !AddrIsGranuleAligned(s, top) {
        (Err(RmiStatusCode::RMI_ERROR_INPUT), s)
    } else if UInt(base) == UInt(walk_top_pre) {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if rec.dev_mem_flags.coh == RmmDevMemCoherence::DEV_MEM_NON_COHERENT
        && !RttEntriesInRangeMemAttr(
            s,
            RttAt(s, walk.rtt_addr),
            walk.level,
            base,
            walk_top_pre,
            RmmRttMemAttr::MEMATTR_NON_CACHEABLE,
        )
    {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if rec.dev_mem_flags.coh == RmmDevMemCoherence::DEV_MEM_NON_COHERENT
        && !RttEntriesInRangeNonCohDevMem(
            s,
            RttAt(s, walk.rtt_addr),
            walk.level,
            base,
            walk_top_pre,
        )
    {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if rec.dev_mem_flags.coh == RmmDevMemCoherence::DEV_MEM_COHERENT
        && !RttEntriesInRangeMemAttr(
            s,
            RttAt(s, walk.rtt_addr),
            walk.level,
            base,
            walk_top_pre,
            RmmRttMemAttr::MEMATTR_PASSTHROUGH,
        )
    {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if rec.dev_mem_flags.coh == RmmDevMemCoherence::DEV_MEM_COHERENT
        && !RttEntriesInRangeCohDevMem(
            s,
            RttAt(s, walk.rtt_addr),
            walk.level,
            base,
            walk_top_pre,
        )
    {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if !RttEntriesInRangeOutputContiguous(
        s,
        RttAt(s, walk.rtt_addr),
        walk.level,
        base,
        walk_top_pre,
        rec.dev_mem_pa,
    ) {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else if AddrRangeIsAuxLive(s, base, top, realm_pre) {
        (Err(RmiStatusCode::RMI_ERROR_RTT(walk.level)), s)
    } else {
        // Success conditions
        let out_top = MinAddress(top, walk_top_pre);
        let s_post = update_rec_dev_mem(
            update_rtte_ripas(s, walk.rtt_addr, walk.level, base, walk_top_pre),
            rec_ptr,
            out_top,
            ToAddress(UInt(pa_pre) + (UInt(walk_top_pre) - UInt(base))),
        );
        (Ok(out_top), s_post)
    }
}
```

This specification function:
1. **Extracts context values** according to the command specification
2. **Checks failure conditions in order**, returning errors with appropriate status codes
3. **On success**, returns