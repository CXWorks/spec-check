pub open spec fn RMI_VSMMU_DESTROY_spec(
    s: S,
    rd: Address,
    vsmmu_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let granule_vsmmu = GranuleAt(s, vsmmu_ptr);
    let granule_rd = GranuleAt(s, rd);

    ((!ImplFeatures(s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED(s)) as bool) && (
    !AddrIsGranuleAligned(s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (
    !PaIsDelegable(s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (
    granule_vsmmu.state != VSMMU(s) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (
    !AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (
    !PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (granule_rd.state
        != RD(s) ==> ResultEqual(result, RMI_ERROR_INPUT(s)) as bool) && (VsmmuIsLive(s, vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_DEVICE(s)) as bool) && (result.is_Ok() ==> (GranuleAt(
        s,
        vsmmu_ptr,
    ).state == DELEGATED(s) && RealmAt(s, rd).num_vsmmus == realm_pre.num_vsmmus - 1)))
}