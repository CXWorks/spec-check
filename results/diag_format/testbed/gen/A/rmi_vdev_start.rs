pub open spec fn rmi_vdev_start_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da == FEATURE_FALSE
        ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && !PaIsDelegable(old_s, rd))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && GranuleAt(old_s, rd).state != RD)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && !AddrIsGranuleAligned(old_s, vdev_ptr))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && !PaIsDelegable(old_s, vdev_ptr))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && GranuleAt(old_s, vdev_ptr).state != VDEV)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm != rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED)
        ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE)
        ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Err()
        ==> (VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op
            && VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED
        && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
        ==> (result.is_Ok()
            && VdevAt(new_s, vdev_ptr).op == VDEV_OP_START
            && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}
