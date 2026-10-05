pub open spec fn sbi_hart_get_status_spec(hartid: UInt64, ret: SbiRet, old_s: S, new_s: S) -> bool {
    (!IsValidHartId(old_s, hartid) ==> (ret.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (IsValidHartId(old_s, hartid) ==> (ret.error == SBI_SUCCESS && IsValidHsmStateId(ret.value) && new_s == old_s))
}
