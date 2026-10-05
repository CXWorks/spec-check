pub open spec fn drtm_version_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (DrtmIsSupported() ==> (result[31] == 0 && result[30:16] == 1 && result[15:0] == 4))
}