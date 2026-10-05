pub open spec fn sbi_cppc_write_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (IsReservedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!IsImplementedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (ResultEqual(result, SBI_SUCCESS) ==> CppcReg(old_s, cppc_reg_id) == val)
    && (ResultEqual(result, SBI_SUCCESS) ==> CppcReg(new_s, cppc_reg_id) == val)
}