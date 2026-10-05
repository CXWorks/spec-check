pub open spec fn ffa_features_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (Bits64(old_s.cmd_input_id as int, 31, 31) == 1 && !IsImplementedFfaFunction(old_s.cmd_input_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (Bits64(old_s.cmd_input_id as int, 31, 31) == 0 && !IsSupportedFrameworkFeature(Bits64(old_s.cmd_input_id as int, 7, 0)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> iface_props(new_s, 2, 3) == QueriedInterfaceProperties(old_s.cmd_input_id, old_s.cmd_input_props))
}