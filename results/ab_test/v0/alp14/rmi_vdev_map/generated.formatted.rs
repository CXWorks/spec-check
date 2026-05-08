pub open spec fn RMI_VDEV_MAP_spec(s: S, rd: Address, vdev_ptr: Address, ipa: Address, level: int, addr: Address) -> bool {
    let realm = RealmAt(s, rd);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let pa_top = ToAddress(UInt(addr) + RttLevelSize(s, level));
    
    // Failure conditions - rd_align
    (!AddrIsGranuleAligned(s, rd) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - rd_bound
    (!PaIsDelegable(s, rd) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - rd_state
    (GranuleAt(s, rd).state != RD ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - vdev_align
    (!AddrIsGranuleAligned(s, vdev_ptr) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - vdev_bound
    (!PaIsDelegable(s, vdev_ptr) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - vdev_gran_state
    (GranuleAt(s, vdev_ptr).state != VDEV ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - vdev_realm
    (vdev_pre.realm != rd ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - level_bound
    ((!RttLevelIsValid(s, realm, level) || level < 2) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - ipa_align
    (!AddrIsRttLevelAligned(s, ipa, level) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - ipa_bound
    (!AddrIsProtected(s, ipa, realm) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - addr_align
    (!AddrIsGranuleAligned(s, addr) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - addr_bound
    (!PaIsDelegableDevMem(s, addr) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - addr_state
    (!GranulesAllState(s, addr, pa_top, DELEGATED) ==> {
        let result = Err(RMI_ERROR_INPUT);
        ResultEqual(result, RMI_ERROR_INPUT)
    }) &&
    
    // Failure conditions - rtt_walk
    (walk.level < level ==> {
        let result = Err(RMI_ERROR_RTT);
        ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0() == walk.level
    }) &&
    
    // Failure conditions - rtte_state
    (walk.rtte.state != UNASSIGNED ==> {
        let result = Err(RMI_ERROR_RTT);
        ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0() == walk.level
    }) &&
    
    // Success conditions - state
    (GranulesAllState(s, addr, pa_top, DEV_MAPPED)) &&
    
    // Success conditions - rtte_state
    (walk.rtte.state == ASSIGNED_DEV) &&
    
    // Success conditions - rtte_addr
    (walk.rtte.addr == addr) &&
    
    // Success conditions - rtte_attr_ncoh
    ((PaIsDelegableNonCohDevMem(s, addr) ==> walk.rtte.attr_prot == MEMATTR_NON_CACHEABLE)) &&
    
    // Success conditions - rtte_attr_coh
    ((PaIsDelegableCohDevMem(s, addr) ==> walk.rtte.attr_prot == MEMATTR_PASSTHROUGH)) &&
    
    // Success conditions - rtte_sh_ncoh
    ((PaIsDelegableNonCohDevMem(s, addr) ==> walk.rtte.sh == SHAREABILITY_OUTER)) &&
    
    // Success conditions - rtte_sh_coh
    ((PaIsDelegableCohDevMem(s, addr) ==> walk.rtte.sh == SHAREABILITY_INNER)) &&
    
    // Success conditions - num_map
    (let vdev = VdevAt(s, vdev_ptr);
     vdev.num_map == vdev_pre.num_map + (RttLevelSize(s, level) << RMM_GRANULE_SIZE_ORDER))
}