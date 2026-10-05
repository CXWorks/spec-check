pub open spec fn sbi_hart_get_status_spec(hartid: UInt64, error: SbiErrorCode, value: UInt64, old_s: S, new_s: S) -> bool {
  (!IsValidHartId(old_s, hartid) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (IsValidHartId(old_s, hartid) ==> IsHsmStateId(value))
  && (IsValidHartId(old_s, hartid) ==> ValueIsHsmStateOfHartDuringCall(old_s, hartid, value))
  && ((IsValidHartId(old_s, hartid))
    ==> error == SBI_SUCCESS)
}