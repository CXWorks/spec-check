pub open spec fn RMI_VDEV_UNLOCK_spec(old_s: S, new_s: S, rd: Address, vdev_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let old_realm = RealmAt(old_s, rd);
    let old_vdev = VdevAt(old_s, vdev_ptr);
    let new_vdev = VdevAt(new_s, vdev_ptr);
    
    (
        (!ImplFeatures().feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (old_vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        ((old_vdev.vdev_state != VDEV_LOCKED && old_vdev.vdev_state != VDEV_STARTED && old_vdev.vdev_state != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
        (old_vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
        (old_vdev.num_map != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
        (
            !ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) && 
            !ResultEqual(result, RMI_ERROR_INPUT) && 
            !ResultEqual(result, RMI_ERROR_DEVICE) ==>
            (
                new_vdev.dma_state == VDEV_DMA_DISABLED &&
                new_vdev.op == VDEV_OP_UNLOCK &&
                new_vdev.comm_state == DEV_COMM_PENDING
            )
        )
    )
}