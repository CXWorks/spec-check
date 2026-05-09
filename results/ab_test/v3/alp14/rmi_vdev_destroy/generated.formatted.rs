pub open spec fn rmi_vdev_destroy_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let pdev_pre = PdevAt(old_s, pdev_ptr);
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let pdev = PdevAt(new_s, pdev_ptr);

    (!ImplFeatures().feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (
    !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(pdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        vdev_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, vdev_ptr).state != VDEV
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (vdev_pre.realm != rd ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && (vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE)) && ((
    vdev_pre.vdev_state != VDEV_NEW && vdev_pre.vdev_state != VDEV_UNLOCKED && vdev_pre.vdev_state
        != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (vdev_pre.num_map != 0
        ==> ResultEqual(result, RMI_ERROR_DEVICE)) && ((ImplFeatures().feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(
        old_s,
        pdev_ptr,
    ).state == PDEV && AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && GranuleAt(
        old_s,
        vdev_ptr,
    ).state == VDEV && vdev_pre.realm == rd && vdev_pre.pdev == pdev_ptr && (vdev_pre.vdev_state
        == VDEV_NEW || vdev_pre.vdev_state == VDEV_UNLOCKED || vdev_pre.vdev_state == VDEV_ERROR)
        && vdev_pre.num_map == 0) ==> (!result.is_Err() && GranuleAt(new_s, vdev_ptr).state
        == DELEGATED && AuxStateEqual32(vdev_pre.aux, vdev_pre.num_aux, DELEGATED) && VdevIdIsFree(
        new_s,
        realm,
        vdev_pre.vdev_id,
    ) && TdiIdIsFree(new_s, vdev_pre.tdi_id, pdev_pre.segment_id) && realm.num_vdevs
        == realm_pre.num_vdevs - 1 && pdev.num_vdevs == pdev_pre.num_vdevs - 1 && (vdev_pre.vsmmu
        == FEATURE_TRUE ==> VsidIsFree(new_s, VsmmuAt(new_s, vdev_pre.vsmmu_addr), vdev_pre.vsid))))
}