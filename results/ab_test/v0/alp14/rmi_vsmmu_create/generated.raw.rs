```verus
pub open spec fn RMI_VSMMU_CREATE_spec(s: S, rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let realm_pre = RealmAt(s, rd);
    let realm = RealmAt(s, rd);
    let vsmmu = VsmmuAt(s, vsmmu_ptr);
    let params = RmiVsmmuParamsAt(s, params_ptr);
    
    // Failure conditions
    (if !ImplFeatures(s).feat_da {
        ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
    } else if !AddrIsGranuleAligned(s, rd) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegable(s, rd) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RD {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if realm_pre.state != REALM_NEW {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(s, vsmmu_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegableDram(s, vsmmu_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, vsmmu_ptr).state != DELEGATED {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(s, params_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !GranuleAccessPermitted(s, params_ptr, PAS_NS) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !RmiVsmmuParamsIsValid(s, params_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(s, params.reg_base) || !AddrIsGranuleAligned(s, params.reg_top) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsProtected(s, params.reg_base, realm) || !AddrIsProtected(s, params.reg_top, realm) || UInt(params.reg_top) <= UInt(params.reg_base) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else {
        // Success conditions
        result.is_Ok() &&
        GranuleAt(s, vsmmu_ptr).state == VSMMU &&
        vsmmu.state == VSMMU_INACTIVE &&
        vsmmu.realm == rd &&
        vsmmu.reg_base == params.reg_base &&
        vsmmu.reg_top == params.reg_top &&
        vsmmu.aidr == params.aidr &&
        vsmmu.idr[0] == params.idr[0] &&
        vsmmu.idr[1] == params.idr[1] &&
        vsmmu.idr[2] == params.idr[2] &&
        vsmmu.idr[3] == params.idr[3] &&
        vsmmu.idr[4] == params.idr[4] &&
        vsmmu.idr[5] == params.idr[5] &&
        vsmmu.idr[6] == params.idr[6] &&
        realm.num_vsmmus == realm_pre.num_vsmmus + 1
    }
}
```