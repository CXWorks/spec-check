pub open spec fn ffa_id_get_spec(id: uint32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_ID_GET) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> id == CallerId())
  && (result == FFA_SUCCESS ==> IsNonSecurePhysicalInstance() ==> id == 0)
  && (result == FFA_SUCCESS ==> 0 == 0)
  && ((!(IsImplementedAtInstance(old_s, FFA_ID_GET)))
    ==> ResultEqual(result, FFA_SUCCESS))
  && (result != FFA_SUCCESS
    ==> id == CallerId())
  && (result != FFA_SUCCESS
    ==> IsNonSecurePhysicalInstance() ==> id == 0)
  && (!(result == FFA_SUCCESS && (IsNonSecurePhysicalInstance()))
    ==> id == 0)
}