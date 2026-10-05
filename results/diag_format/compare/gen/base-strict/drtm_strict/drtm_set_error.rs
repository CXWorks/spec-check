pub open spec fn drtm_set_error_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (DrtmErrorPreviouslySet() ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> DrtmErrorPreviouslySet())
    && (ResultEqual(result, SUCCESS) ==> PersistedInNonVolatileSecureStorage(PersistedDrtmError()))
    && (!DceStageCompleted() ==> (ResultEqual(result, SUCCESS) ==> PersistedDrtmError() == error_code))
    && (DceStageCompleted() ==> (ResultEqual(result, SUCCESS) ==> (Bits(PersistedDrtmError(), 63, 11) == Bits(error_code, 63, 11) && Bits(PersistedDrtmError(), 2, 0) == DRTM_ERROR_PHASE_DLME && Bits(PersistedDrtmError(), 10, 3) == 0xFF)))
}