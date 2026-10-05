pub open spec fn drtm_close_locality_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && ((locality != 2 && locality != 3) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (LocalityIsClosed(locality) ==> ResultEqual(result, ALREADY_CLOSED))
    && (!LocalityIsRelinquished(locality) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> LocalityIsClosed(locality))
    && (ResultEqual(result, SUCCESS) ==> TpmLocalityState(new_s, locality) == TpmLocalityState(old_s, locality))
}