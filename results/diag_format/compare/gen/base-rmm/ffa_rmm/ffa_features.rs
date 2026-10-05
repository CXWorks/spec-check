pub open spec fn ffa_features_spec(result: UInt32, interface_properties: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidFunctionOrFeatureId(old_s, w1(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsImplementedFunctionOrFeature(old_s, w1(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> interface_properties == PropertiesOf(w1(old_s)))
}