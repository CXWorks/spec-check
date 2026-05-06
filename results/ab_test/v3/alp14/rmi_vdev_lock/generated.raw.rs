```verus
pub open spec fn RMI_VDEV_LOCK_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    
    (
        // Failure: da_supp
        (!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
        // Failure: rd_align
        (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: rd_bound
        (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: rd_state
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vdev_align
        (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vdev_bound
        (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vdev_gran_state
        (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vdev_realm
        (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vdev_state
        (vdev.vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
        // Failure: comm_state
        (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
        // Success conditions
        (
            (ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() &&
             AddrIsGranuleAligned(old_s, rd) &&
             PaIsDelegable(old_s, rd) &&
             GranuleAt(old_s, rd).state == RD &&
             AddrIsGranuleAligned(old_s, vdev_ptr) &&
             PaIsDelegable(old_s, vdev_ptr) &&
             GranuleAt(old_s, vdev_ptr).state == VDEV &&
             vdev.realm == rd &&
             vdev.vdev_state == VDEV_UNLOCKED &&
             vdev.comm_state == DEV_COMM_IDLE)
            ==>
            (result.is_Ok() &&
             VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK &&
             VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
        )
    )
}
```