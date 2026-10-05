pub open spec fn drtm_close_locality_spec(locality: UInt32, result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (locality != 2 && locality != 3 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (LocalityIsClosed(old_s, locality) ==> ResultEqual(result, ALREADY_CLOSED))
  && (!LocalityIsRelinquished(old_s, locality) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> LocalityIsClosed(new_s, locality))
  && ((DrtmIsSupported(old_s) &&
       !(locality != 2 && locality != 3) &&
       !(LocalityIsClosed(old_s, locality)) &&
       LocalityIsRelinquished(old_s, locality))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> LocalityIsClosed(new_s, locality))
}