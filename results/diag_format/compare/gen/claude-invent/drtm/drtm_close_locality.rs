pub open spec fn drtm_close_locality_spec(result: i64, function_id: UInt32, locality: UInt32, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && !(locality == 2 || locality == 3)) ==> result == INVALID_PARAMETERS)
    && ((DrtmIsSupported(old_s) && (locality == 2 || locality == 3) && TpmLocalityIsClosed(old_s, locality as int)) ==> result == ALREADY_CLOSED)
    && ((DrtmIsSupported(old_s) && (locality == 2 || locality == 3) && !TpmLocalityIsClosed(old_s, locality as int) && !TpmLocalityIsRelinquished(old_s, locality as int)) ==> result == DENIED)
    && ((result != SUCCESS && (locality == 2 || locality == 3)) ==> (TpmLocalityIsClosed(new_s, locality as int) == TpmLocalityIsClosed(old_s, locality as int)))
    && ((DrtmIsSupported(old_s) && (locality == 2 || locality == 3) && !TpmLocalityIsClosed(old_s, locality as int) && TpmLocalityIsRelinquished(old_s, locality as int)) ==> (result == SUCCESS && TpmLocalityIsClosed(new_s, locality as int)))
}
