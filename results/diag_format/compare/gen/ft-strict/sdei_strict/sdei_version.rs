pub open spec fn sdei_version_spec(result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (SdeiIsSupported(old_s) ==> Bits(result, 63, 63) == 0)
  && (SdeiIsSupported(old_s) ==> Bits(result, 62, 48) == 1)
  && (SdeiIsSupported(old_s) ==> Bits(result, 47, 32) == 1)
  && (SdeiIsSupported(old_s) ==> Bits(result, 31, 0) == VendorDefinedVersion())
  && (SdeiIsSupported(old_s) ==> SdeiImplementsAllCalls())
  && ((SdeiIsSupported(old_s))
    ==> result == result)
}