pub open spec fn rmi_vsmmu_features_spec(features_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsAligned(old_s, features_ptr, 0x100) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, features_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((AddrIsAligned(old_s, features_ptr, 0x100) &&
       SecureAccessPermitted(old_s, features_ptr))
    ==> result.is_Ok())
}