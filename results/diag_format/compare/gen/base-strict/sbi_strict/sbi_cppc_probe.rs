pub open spec fn sbi_cppc_probe_spec(result: sbiret, cppc_reg_id: UInt32, old_s: S, new_s: S) -> bool {
    (IsReservedCppcRegId(cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (CppcProbeFailed(cppc_reg_id) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> IsCppcRegImplemented(cppc_reg_id) ==> value == CppcRegWidth(cppc_reg_id))
    && (ResultEqual(result, SBI_SUCCESS) ==> !IsCppcRegImplemented(cppc_reg_id) ==> value == 0)
    && (old_s == new_s)
}