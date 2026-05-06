pub open spec fn RMI_VSMMU_CREATE_spec(
    s: S,
    rd: Address,
    vsmmu_ptr: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode,
    new_s: S,
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let realm = RealmAt(new_s, rd);
    let vsmmu = VsmmuAt(new_s, vsmmu_ptr);
    let params = RmiVsmmuParamsAt(s, params_ptr);
    let granule_before = GranuleAt(s, vsmmu_ptr);
    let granule_after = GranuleAt(new_s, vsmmu_ptr);

    ((!ImplFeatures(s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (granule_before.state
        != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(params_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(s, params_ptr, PAS_NS)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiVsmmuParamsIsValid(s, params_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(params.reg_base)
        || !AddrIsGranuleAligned(params.reg_top)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsProtected(s, params.reg_base, realm_pre) || !AddrIsProtected(
        s,
        params.reg_top,
        realm_pre,
    ) || (params.reg_top as int) <= (params.reg_base as int)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (result.is_Ok() ==> (granule_after.state == VSMMU && vsmmu.state == VSMMU_INACTIVE
        && vsmmu.realm == rd && vsmmu.reg_base == params.reg_base && vsmmu.reg_top == params.reg_top
        && vsmmu.aidr == params.aidr && vsmmu.idr[0] == params.idr[0] && vsmmu.idr[1]
        == params.idr[1] && vsmmu.idr[2] == params.idr[2] && vsmmu.idr[3] == params.idr[3]
        && vsmmu.idr[4] == params.idr[4] && vsmmu.idr[5] == params.idr[5] && vsmmu.idr[6]
        == params.idr[6] && realm.num_vsmmus == realm_pre.num_vsmmus + 1)))
}