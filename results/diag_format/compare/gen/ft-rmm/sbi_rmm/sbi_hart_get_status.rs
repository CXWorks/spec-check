pub open spec fn sbi_hart_get_status_spec(hartid: unsigned long, error: long, value: long, old_s: S, new_s: S) -> bool {
  (!IsValidHartid(old_s, hartid) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (IsHsmStateId(value))
  && ((IsValidHartid(old_s, hartid))
    ==> error == 0)
}