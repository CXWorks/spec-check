pub open spec fn rmi_vdev_unlock_spec(result: Result<(), RmiStatusCode>, rd: Address, vdev_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((vdev.vdev_state != VDEV_LOCKED
         && vdev.vdev_state != VDEV_STARTED
         && vdev.vdev_state != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev.num_map != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
         && AddrIsGranuleAligned(rd)
         && PaIsDelegable(rd)
         && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(vdev_ptr)
         && PaIsDelegable(vdev_ptr)
         && GranuleAt(old_s, vdev_ptr).state == VDEV
         && vdev.realm == rd
         && (vdev.vdev_state == VDEV_LOCKED
             || vdev.vdev_state == VDEV_STARTED
             || vdev.vdev_state == VDEV_ERROR)
         && vdev.comm_state == DEV_COMM_IDLE
         && vdev.num_map == 0)
        ==> (result.is_Ok()
             && VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED
             && VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK
             && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
    )
}