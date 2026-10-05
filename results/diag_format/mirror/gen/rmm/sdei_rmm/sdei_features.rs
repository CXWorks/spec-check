pub open spec fn sdei_features_spec(feature: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidSdeiFeature(old_s, feature) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SDEI_SUCCESS && feature == BIND_SLOTS ==> result[63:32] == 0)
  && (result == SDEI_SUCCESS && feature == BIND_SLOTS ==> result[31:16] == SharedEventSlotCount(new_s))
  && (result == SDEI_SUCCESS && feature == BIND_SLOTS ==> result[15:0] == PrivateEventSlotCount(new_s))
  && (result == SDEI_SUCCESS && feature == RELATIVE_MODE && IsRelativeModeSupported(old_s) ==> result == 1)
  && (result == SDEI_SUCCESS && feature == RELATIVE_MODE && !IsRelativeModeSupported(old_s) ==> result == 0)
  && ((IsSdeiSupported(old_s) &&
       IsValidSdeiFeature(old_s, feature))
    ==> result == SDEI_SUCCESS)
}