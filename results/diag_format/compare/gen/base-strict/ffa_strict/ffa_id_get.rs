pub open spec fn ffa_id_get_spec(result: UInt32, id: UInt16, reserved: UInt16, error_code: Int32, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplementedAtInstance(FFA_ID_GET, CurrentFfaInstance()) ==> ResultEqual(error_code, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> id == CallerFfaId())
    && (ResultEqual(result, FFA_SUCCESS) ==> reserved == 0)
    && (IsNonSecurePhysicalFfaInstance(CurrentFfaInstance()) ==> (ResultEqual(result, FFA_SUCCESS) ==> id == 0))
    && (old_s == new_s)
}