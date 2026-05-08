```rust
pub open spec fn RMI_PDEV_CREATE_spec(
    s: S,
    pdev_ptr: Address,
    params_ptr: Address,
) -> (result: Result<(), RmiStatusCode>)
{
    let pdev = PdevAt(s, pdev_ptr);
    let params = RmiPdevParamsAt(s, params_ptr);
    
    // Failure conditions in order of precedence
    if !ImplFeatures(s).feat_da {
        Err(RMI_ERROR_NOT_SUPPORTED)
    } else if !AddrIsGranuleAligned(s, pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegableDram(s, pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, pdev_ptr).state != DELEGATED {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(s, params_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !GranuleAccessPermitted(s, params_ptr, PAS_NS) {
        Err(RMI_ERROR_INPUT)
    } else if !RmiPdevParamsIsValid(s, params_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !RmiPdevFlagsSupported(s, params.flags) {
        Err(RMI_ERROR_INPUT)
    } else if params.num_aux != VdevAuxCount(s, params.flags, RmiVdevFlags::default()) {
        Err(RMI_ERROR_INPUT)
    } else if !AuxAligned32(s, params.aux, params.num_aux) {
        Err(RMI_ERROR_INPUT)
    } else if AuxAlias32(s, pdev_ptr, params.aux, params.num_aux) {
        Err(RMI_ERROR_INPUT)
    } else if !AuxStateEqual32(s, params.aux, params.num_aux, DELEGATED) {
        Err(RMI_ERROR_INPUT)
    } else {
        // Success conditions are postconditions
        Ok(())
    }
}
```

**Associated postconditions for the success case:**

```rust
pub open spec fn RMI_PDEV_CREATE_post(
    s_pre: S,
    s_post: S,
    pdev_ptr: Address,
    params_ptr: Address,
) -> bool
{
    let pdev = PdevAt(s_post, pdev_ptr);
    let params = RmiPdevParamsAt(s_pre, params_ptr);
    
    && GranuleAt(s_post, pdev_ptr).state == PDEV
    && pdev.pdev_id == params.pdev_id
    && pdev.spdm == params.flags.spdm
    && pdev.ncoh_ide == params.flags.ncoh_ide
    && pdev.ncoh_addr == params.flags.ncoh_addr
    && pdev.coh_ide == params.flags.coh_ide
    && pdev.coh_addr == params.flags.coh_addr
    && pdev.segment_id == params.segment_id
    && pdev.ecam_addr == params.ecam_addr
    && pdev.root_id == params.root_id
    && pdev.cert_id == params.cert_id
    && pdev.rid_base == params.rid_base
    && pdev.rid_top == params.rid_top
    && pdev.hash_algo == params.hash_algo
    && pdev.ncoh_ide_sid == params.ncoh_ide_sid
    && pdev.ncoh_num_addr_range == params.ncoh_num_addr_range
    && RmiAddressRangesEqual16(s_post, pdev.ncoh_addr_range, params.ncoh_addr_range, params.ncoh_num_addr_range)
    && pdev.coh_num_addr_range == params.coh_num_addr_range
    && RmiAddressRangesEqual4(s_post, pdev.coh_addr_range, params.coh_addr_range, params.coh_num_addr_range)
    && pdev.state == PDEV_NEW
    && pdev.comm_state == DEV_COMM_PENDING
    && pdev.num_vdevs == 0
    && AuxEqual32(s_post, pdev.aux, params.aux, VdevAuxCount(s_pre, params.flags, RmiVdevFlags::default()))
    && pdev.num_aux == VdevAuxCount(s_pre, params.flags, RmiVdevFlags::default())
    && AuxStateEqual32(s_post, pdev.aux, VdevAuxCount(s_pre, params.flags, RmiVdevFlags::default()), PDEV_AUX)
    && pdev.p2p_enabled == params.flags.p2p
    && pdev.p2p_stream_valid == RMM_FALSE
}
```