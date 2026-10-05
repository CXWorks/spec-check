pub open spec fn drtm_set_error_spec(error_code: Int64, result: Result<(), DrtmStatusCode>, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (DrtmErrorPreviouslySet(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> DrtmErrorPreviouslySet(new_s))
  && (result == SUCCESS ==> PersistedInNonVolatileSecureStorage(new_s, PersistedDrtmError()))
  && (result == SUCCESS ==> !DceStageCompleted(new_s) ==> PersistedDrtmError(new_s) == error_code)
  && (result == SUCCESS ==> DceStageCompleted(new_s) ==> Bits(PersistedDrtmError(new_s), 63, 11) == Bits(error_code, 63, 11))
  && (result == SUCCESS ==> DceStageCompleted(new_s) ==> Bits(PersistedDrtmError(new_s), 2, 0) == DRTM_ERROR_PHASE_DLME)
  && (result == SUCCESS ==> DceStageCompleted(new_s) ==> Bits(PersistedDrtmError(new_s), 10, 3) == 0xFF)
  && ((DrtmIsSupported(old_s) &&
       !DrtmErrorPreviouslySet(old_s))
    ==> result == SUCCESS)
}