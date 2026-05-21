pub open spec fn rmi_vsmmu_destroy_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    vsmmu_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);

    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(vsmmu_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    VsmmuIsLive(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (ImplFeatures(
        old_s,
    ).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(
        old_s,
        rd,
    ).state == RD && AddrIsGranuleAligned(vsmmu_ptr) && PaIsDelegable(vsmmu_ptr) && GranuleAt(
        old_s,
        vsmmu_ptr,
    ).state == VSMMU && !VsmmuIsLive(old_s, vsmmu_ptr) ==> (result == RMI_SUCCESS && GranuleAt(
        new_s,
        vsmmu_ptr,
    ).state == DELEGATED && realm.num_vsmmus == realm_pre.num_vsmmus - 1))
}