```verus
pub open spec fn RMI_VDEV_GET_STATE_spec(
    old_s: S,
    new_s: S,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: u8,
) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);
    
    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    &&
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    &&
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    &&
    // Failure condition: vdev_gran_state
    (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    &&
    // Success condition: state
    (result.is_Ok() ==> state == vdev.vdev_state)
    &&
    // No footprint - state unchanged
    new_s == old_s
}
```