pub open spec fn rmi_vdev_lock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (
        (ImplFeatures(old_s).feat_da != FEATURE_TRUE && AddrIsGranuleAligned(old_s, rd))
        ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
    )
    && (
        (ImplFeatures(old_s).feat_da != FEATURE_TRUE && !AddrIsGranuleAligned(old_s, rd))
        ==> (ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) || ResultEqual(result, RMI_ERROR_INPUT))
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && (!AddrIsGranuleAligned(old_s, rd)
                || !PaIsDelegable(old_s, rd)
                || GranuleAt(old_s, rd).state != RD))
        ==> ResultEqual(result, RMI_ERROR_INPUT)
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && (!AddrIsGranuleAligned(old_s, vdev_ptr)
                || !PaIsDelegable(old_s, vdev_ptr)
                || GranuleAt(old_s, vdev_ptr).state != VDEV))
        ==> ResultEqual(result, RMI_ERROR_INPUT)
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, vdev_ptr)
            && PaIsDelegable(old_s, vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && VdevAt(old_s, vdev_ptr).realm != rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, vdev_ptr)
            && PaIsDelegable(old_s, vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && VdevAt(old_s, vdev_ptr).realm == rd
            && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED)
        ==> ResultEqual(result, RMI_ERROR_DEVICE)
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, vdev_ptr)
            && PaIsDelegable(old_s, vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && VdevAt(old_s, vdev_ptr).realm == rd
            && VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE)
        ==> ResultEqual(result, RMI_ERROR_DEVICE)
    )
    && (
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, vdev_ptr)
            && PaIsDelegable(old_s, vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && VdevAt(old_s, vdev_ptr).realm == rd
            && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_UNLOCKED
            && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
        ==> (
            result.is_Ok()
            && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK
            && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING
            && VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state
            && VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm
            && VdevAt(new_s, vdev_ptr).pdev == VdevAt(old_s, vdev_ptr).pdev
            && GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state
            && GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state
        )
    )
}
