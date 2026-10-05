pub open spec fn ffa_features_spec(id: UInt32, input_props: UInt32, result: UInt32, iface_props: [UInt32; 2], old_s: S, new_s: S) -> bool {
  ((id & 0x1) == 1 && !IsImplementedFfaFunction(old_s, id) ==> ResultEqual(result, NOT_SUPPORTED))
  && ((id & 0x1) == 0 && !IsSupportedFrameworkFeature(old_s, id & 0xff) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> iface_props[0] == QueriedInterfaceProperties(old_s, id, input_props)[0])
  && (result == FFA_SUCCESS ==> iface_props[1] == QueriedInterfaceProperties(old_s, id, input_props)[1])
  && ((!((id & 0x1) == 1 && !IsImplementedFfaFunction(old_s, id)) &&
       !((id & 0x1) == 0 && !IsSupportedFrameworkFeature(old_s, id & 0xff)))
    ==> ResultEqual(result, FFA_SUCCESS))
}