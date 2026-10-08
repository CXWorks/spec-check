pub open spec fn rmi_vdev_unlock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (AddrIsGranuleAligned(old_s, rd) ==> true)
    && (PaIsDelegable(old_s, rd) ==> true)
    && (GranuleAt(old_s, rd).state == RD ==> true)
    && (AddrIsGranuleAligned(old_s, vdev_ptr) ==> true)
    && (PaIsDelegable(old_s, vdev_ptr) ==> true)
    && (GranuleAt(old_s, vdev_ptr).state == VDEV ==> true)
    && (VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> true)
    && (vdev_state(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_comm_state(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_mappings(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED && VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}

pub open spec fn vdev_state(vdev_ptr: Address, s: S) -> bool {
    let vdev = VdevAt(s, vdev_ptr);
    vdev.state != VDEV_LOCKED && vdev.state != VDEV_STARTED && vdev.state != VDEV_ERROR
}

pub open spec fn vdev_comm_state(vdev_ptr: Address, s: S) -> bool {
    let vdev = VdevAt(s, vdev_ptr);
    vdev.comm_state != DEV_COMM_IDLE
}

pub open spec fn vdev_mappings(vdev_ptr: Address, s: S) -> bool {
    let vdev = VdevAt(s, vdev_ptr);
    vdev.num_map != 0
}