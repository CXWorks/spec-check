pub open spec fn drtm_close_locality_spec(result: Int64, locality: UInt32, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (locality != 2 && locality != 3 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (TpmLocalityIsClosed(old_s, locality) ==> ResultEqual(result, ALREADY_CLOSED))
    && (!TpmLocalityIsRelinquished(old_s, locality) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> TpmLocalityIsClosed(new_s, locality))
}