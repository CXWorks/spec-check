pub open spec fn ffa_id_get_spec(result: UInt32, id: UInt16, reserved: UInt16, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplementedAtInstance(old_s, FFA_ID_GET, CurrentFfaInstance(old_s)) ==> ResultEqual(error_code, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> id == CallerFfaId())
  && (result == FFA_SUCCESS ==> reserved == 0)
  && (result == FFA_SUCCESS ==> IsNonSecurePhysicalFfaInstance(old_s, CurrentFfaInstance(old_s)) ==> id == 0)
  && ((!(IsFunctionImplementedAtInstance(old_s, FFA_ID_GET, CurrentFfaInstance(old_s))))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> id == 0)
  && (result != FFA_SUCCESS
    ==> reserved == 0)
}