pub open spec fn drtm_version_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (DrtmIsSupported() ==> (Bits(result, 31, 31) == 0 && Bits(result, 30, 16) == 1 && Bits(result, 15, 0) == 4 && AllNonOptionalDrtmFunctionsImplemented()))
    && (old_s == new_s)
}