pub open spec fn sdei_version_spec(result: Result<int, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> result[63] == 0)
  && (result.is_Ok() ==> result[62:48] == 1)
  && (result.is_Ok() ==> result[47:32] == 1)
  && (result.is_Ok() ==> result[31:0] == VendorDefinedVersion())
  && (result.is_Ok() ==> SdeiImplementsAllCalls())
  && ((SdeiIsSupported(old_s))
    ==> result.is_Ok())
}