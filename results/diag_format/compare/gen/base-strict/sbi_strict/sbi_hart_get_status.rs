pub open spec fn sbi_hart_get_status_spec(error: SbiErrorCode, value: UInt64, hartid: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidHartId(hartid) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (IsValidHartId(hartid) ==> (IsHsmStateId(value) && ValueIsHsmStateOfHartDuringCall(hartid, value)))
    && (old_s == new_s)
}