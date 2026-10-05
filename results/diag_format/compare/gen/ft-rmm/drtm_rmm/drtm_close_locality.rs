pub open spec fn drtm_close_locality_spec(locality: UInt32, result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (locality != 2 && locality != 3 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (TpmLocalityIsClosed(old_s, locality) ==> ResultEqual(result, ALREADY_CLOSED))
  && (!TpmLocalityIsRelinquished(old_s, locality) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> TpmLocalityIsClosed(new_s, locality))
  && ((DrtmIsSupported(old_s) &&
       !(locality != 2 && locality != 3) &&
       !(TpmLocalityIsClosed(old_s, locality)) &&
       TpmLocalityIsRelinquished(old_s, locality))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> TpmLocalityIsClosed(new_s, locality) == TpmLocalityIsClosed(old_s, locality))
}