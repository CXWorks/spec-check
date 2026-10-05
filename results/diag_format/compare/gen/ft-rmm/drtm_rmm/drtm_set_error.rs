pub open spec fn drtm_set_error_spec(error_code: Int64, result: Result<(), DrtmStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (IsDrtmErrorSet(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> IsDrtmErrorSet(new_s))
  && (result == SUCCESS && !IsDlmePhase(old_s) ==> StoredDrtmError(new_s) == error_code)
  && (result == SUCCESS && IsDlmePhase(old_s) ==> StoredDrtmError(new_s)[2:0] == DLME)
  && (result == SUCCESS && IsDlmePhase(old_s) ==> StoredDrtmError(new_s)[10:3] == 0xFF)
  && (result == SUCCESS && IsDlmePhase(old_s) ==> StoredDrtmError(new_s)[63:11] == error_code[63:11])
  && ((IsDrtmSupported(old_s) &&
       !IsDrtmErrorSet(old_s))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> StoredDrtmError(new_s) == StoredDrtmError(old_s))
  && (result != SUCCESS
    ==> IsDrtmErrorSet(new_s) == IsDrtmErrorSet(old_s))
}