pub open spec fn ffa_features_spec(id_type: Bits1, function_id: UInt32, feature_reserved: Bits23, feature_id: UInt8, input_properties: UInt32, result: UInt32, interface_properties: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidFunctionOrFeatureId(old_s, id_type, function_id, feature_reserved, feature_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsImplementedFunctionOrFeature(old_s, id_type, function_id, feature_reserved, feature_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> interface_properties == PropertiesOf(old_s, id_type, function_id, feature_reserved, feature_id))
  && ((IsValidFunctionOrFeatureId(old_s, id_type, function_id, feature_reserved, feature_id) &&
       IsImplementedFunctionOrFeature(old_s, id_type, function_id, feature_reserved, feature_id))
    ==> result == FFA_SUCCESS)
}