pub open spec fn ffa_id_get_spec(result: Result<(), FfaStatusCode>, old_s: S, new_s: S, id: u16) -> bool {
    (!IsImplementedAtInstance(FFA_ID_GET) ==> ResultEqual(result, FFA_NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> (id == CallerId() && IsNonSecurePhysicalInstance() ==> id == 0 && w2(new_s, 31, 16) == 0))
    && (ResultEqual(result, FFA_SUCCESS) ==> old_s == new_s)
}