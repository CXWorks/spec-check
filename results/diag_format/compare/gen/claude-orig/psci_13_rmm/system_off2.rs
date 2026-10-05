pub open spec fn system_off2_spec(fid: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(SYSTEM_OFF2) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AreInputParametersValid() ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((IsFunctionImplemented(SYSTEM_OFF2) && AreInputParametersValid()) ==> true)
}
