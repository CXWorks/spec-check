pub open spec fn drtm_set_error_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsDrtmErrorSet() ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> IsDrtmErrorSet())
    && (ResultEqual(result, SUCCESS) ==> (!IsDlmePhase() ==> StoredDrtmError() == error_code))
    && (ResultEqual(result, SUCCESS) ==> (IsDlmePhase() ==> (StoredDrtmError()[2:0] == DLME)))
    && (ResultEqual(result, SUCCESS) ==> (IsDlmePhase() ==> (StoredDrtmError()[10:3] == 0xFF)))
    && (ResultEqual(result, SUCCESS) ==> (IsDlmePhase() ==> (StoredDrtmError()[63:11] == error_code[63:11])))
}