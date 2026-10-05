pub open spec fn sdei_version_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (SdeiIsSupported() ==> (result[63] == 0 && result[62:48] == 1 && result[47:32] == 1 && result[31:0] == VendorDefinedVersion() && SdeiImplementsAllCalls()))
}