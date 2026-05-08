```verus
pub open spec fn RMI_VDEV_UNLOCK_spec(s: S, rd: Address, vdev_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let realm = RealmAt(s, rd);
    let vdev = VdevAt(s, vdev_ptr);
    let da_supp = ImplFeatures(s).feat_da == FEATURE_TRUE;
    
    let rd_align = AddrIsGranuleAligned(rd);
    let rd_bound = PaIsDelegable(rd);
    let rd_state_ok = GranuleAt(s, rd).state == RD;
    let vdev_align = AddrIsGranuleAligned(vdev_ptr);
    let vdev_bound = PaIsDelegable(vdev_ptr);
    let vdev_gran_state_ok = GranuleAt(s, vdev_ptr).state == VDEV;
    let vdev_realm_ok = vdev.realm == rd;
    let vdev_state_ok = vdev.vdev_state == VDEV_LOCKED || vdev.vdev_state == VDEV_STARTED || vdev.vdev_state == VDEV_ERROR;
    let comm_state_ok = vdev.comm_state == DEV_COMM_IDLE;
    let num_map_ok = vdev.num_map == 0;
    
    if !da_supp {
        result == RMI_ERROR_NOT_SUPPORTED
    } else if !rd_align || !vdev_align {
        result == RMI_ERROR_INPUT
    } else if !rd_bound {
        result == RMI_ERROR_INPUT
    } else if !rd_state_ok {
        result == RMI_ERROR_INPUT
    } else if !vdev_bound {
        result == RMI_ERROR_INPUT
    } else if !vdev_gran_state_ok {
        result == RMI_ERROR_INPUT
    } else if !vdev_realm_ok {
        result == RMI_ERROR_INPUT
    } else if !vdev_state_ok {
        result == RMI_ERROR_DEVICE
    } else if !comm_state_ok {
        result == RMI_ERROR_DEVICE
    } else if !num_map_ok {
        result == RMI_ERROR_DEVICE
    } else {
        result == RMI_SUCCESS && 
        vdev.dma_state == VDEV_DMA_DISABLED &&
        vdev.op == VDEV_OP_UNLOCK &&
        vdev.comm_state == DEV_COMM_PENDING
    }
}
```