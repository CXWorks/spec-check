pub open spec fn ffa_features_spec(function_or_feature_id: UInt32, input_properties: UInt32, result: FfaReturnCode, old_s: S, new_s: S) -> bool {
    (!FfaInterfaceOrFeatureIsImplemented(old_s, function_or_feature_id, input_properties) ==> (IsFfaError(result, NOT_SUPPORTED) && new_s == old_s))
    && (FfaInterfaceOrFeatureIsImplemented(old_s, function_or_feature_id, input_properties) ==> (IsFfaSuccess(result) && new_s == old_s))
}
