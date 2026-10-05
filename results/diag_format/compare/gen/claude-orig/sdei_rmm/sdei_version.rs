pub open spec fn sdei_version_spec(fid: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (SdeiIsSupported() ==> (
        (((result as u64) >> 63u64) == 0u64)
        && ((((result as u64) >> 48u64) & 0x7fffu64) == 1u64)
        && ((((result as u64) >> 32u64) & 0xffffu64) == 1u64)
        && ((((result as u64) & 0xffff_ffffu64) as int) == (VendorDefinedVersion() as int))
        && SdeiImplementsAllCalls()
    ))
    && (new_s == old_s)
}
