pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (!AddrIsGranuleAligned(old_s, rd)
            || !PaIsDelegable(old_s, rd)
            || GranuleAt(old_s, rd).state != RD
            || !AddrIsGranuleAligned(old_s, pdev_ptr)
            || !PaIsDelegable(old_s, pdev_ptr)
            || GranuleAt(old_s, pdev_ptr).state != PDEV
            || !AddrIsGranuleAligned(old_s, vdev_ptr)
            || !PaIsDelegable(old_s, vdev_ptr)
            || GranuleAt(old_s, vdev_ptr).state != VDEV))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && (VdevAt(old_s, vdev_ptr).realm != rd
            || VdevAt(old_s, vdev_ptr).pdev != pdev_ptr
            || !(VdevAt(old_s, vdev_ptr).vdev_state == VDEV_NEW
                || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_UNLOCKED
                || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_ERROR)
            || VdevAt(old_s, vdev_ptr).num_map != 0))
        ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Err() ==> (
        GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state
        && RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs
        && PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && VdevAt(old_s, vdev_ptr).realm == rd
        && VdevAt(old_s, vdev_ptr).pdev == pdev_ptr
        && (VdevAt(old_s, vdev_ptr).vdev_state == VDEV_NEW
            || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_UNLOCKED
            || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_ERROR)
        && VdevAt(old_s, vdev_ptr).num_map == 0)
        ==> (result.is_Ok()
            && GranuleAt(new_s, vdev_ptr).state == DELEGATED
            && AuxStateEqual32(new_s, VdevAt(old_s, vdev_ptr).aux, VdevAt(old_s, vdev_ptr).num_aux as int, DELEGATED)
            && VdevIdIsFree(new_s, RealmAt(new_s, rd), VdevAt(old_s, vdev_ptr).vdev_id)
            && TdiIdIsFree(new_s, VdevAt(old_s, vdev_ptr).tdi_id, PdevAt(old_s, pdev_ptr).segment_id)
            && (RealmAt(new_s, rd).num_vdevs as int) == (RealmAt(old_s, rd).num_vdevs as int) - 1
            && (PdevAt(new_s, pdev_ptr).num_vdevs as int) == (PdevAt(old_s, pdev_ptr).num_vdevs as int) - 1
            && (VdevAt(old_s, vdev_ptr).vsmmu == FEATURE_TRUE
                ==> VsidIsFree(new_s, VsmmuAt(new_s, VdevAt(old_s, vdev_ptr).vsmmu_addr), VdevAt(old_s, vdev_ptr).vsid))))
}
