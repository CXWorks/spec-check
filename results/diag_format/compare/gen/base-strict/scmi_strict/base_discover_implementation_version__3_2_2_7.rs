pub open spec fn base_discover_implementation_version_spec(result: RsiCommandReturnCode, implementation_version: UInt32, old_s: S, new_s: S) -> bool {
    (true ==> result == RSI_SUCCESS)
    && (true ==> implementation_version == VendorImplementationVersion())
    && (true ==> forall|v: UInt32| IsEarlierImplementationVersion(v) ==> v < implementation_version)
    && (true ==> old_s == new_s)
}