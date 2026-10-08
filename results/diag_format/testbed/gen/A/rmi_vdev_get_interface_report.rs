pub open spec fn rmi_vdev_get_interface_report_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    ((ImplFeatures(old_s).feat_da != FEATURE_TRUE) ==> result.is_Err())
    && ((ImplFeatures(old_s).feat_da != FEATURE_TRUE && AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (!AddrIsGranuleAligned(old_s, vdev_ptr)
            || !PaIsDelegable(old_s, vdev_ptr)
            || GranuleAt(old_s, vdev_ptr).state != VDEV)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (!AddrIsGranuleAligned(old_s, rd)
            || !PaIsDelegable(old_s, rd)
            || GranuleAt(old_s, rd).state != RD)) ==> result.is_Err())
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm != rd) ==> result.is_Err())
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (!AddrIsGranuleAligned(old_s, rd)
            || !PaIsDelegable(old_s, rd)
            || GranuleAt(old_s, rd).state != RD
            || (PaIsDelegable(old_s, rd)
                && GranuleAt(old_s, rd).state == RD
                && AddrIsGranuleAligned(old_s, vdev_ptr)
                && PaIsDelegable(old_s, vdev_ptr)
                && GranuleAt(old_s, vdev_ptr).state == VDEV
                && VdevAt(old_s, vdev_ptr).realm != rd))
        && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR
        && (VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_STARTED)
        && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED)
            || VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE)) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED)
            || VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE)) ==> result.is_Err())
    && (result.is_Err() ==> (VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op
        && VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && (VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_STARTED)
        && VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
        ==> (result.is_Ok()
            && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT
            && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING
            && VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state
            && VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm
            && VdevAt(new_s, vdev_ptr).pdev == VdevAt(old_s, vdev_ptr).pdev
            && GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state))
}
