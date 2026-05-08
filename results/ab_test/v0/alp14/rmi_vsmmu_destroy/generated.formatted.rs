pub open spec fn RMI_VSMMU_DESTROY_spec(
    s: S,
    rd: Address,
    vsmmu_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let realm = RealmAt(s, rd);
    let vsmmu = VsmmuAt(s, vsmmu_ptr);

    // Failure conditions
    let da_supp_fails = !ImplFeatures(s).feat_da && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED);
    let rd_align_fails = !AddrIsGranuleAligned(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fails = !PaIsDelegable(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fails = GranuleAt(s, rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let vsmmu_align_fails = !AddrIsGranuleAligned(s, vsmmu_ptr) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let vsmmu_bound_fails = !PaIsDelegable(s, vsmmu_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let vsmmu_state_fails = GranuleAt(s, vsmmu_ptr).state != VSMMU && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let vsmmu_live_fails = VsmmuIsLive(s, vsmmu_ptr) && ResultEqual(result, RMI_ERROR_DEVICE);

    // Success conditions
    let gran_state_success = GranuleAt(s, vsmmu_ptr).state == DELEGATED;
    let num_vsmmus_success = realm.num_vsmmus == realm_pre.num_vsmmus - 1;

    // Either a failure condition holds or all success conditions hold
    da_supp_fails || rd_align_fails || rd_bound_fails || rd_state_fails || vsmmu_align_fails
        || vsmmu_bound_fails || vsmmu_state_fails || vsmmu_live_fails || (result.is_Ok()
        && gran_state_success && num_vsmmus_success)
}