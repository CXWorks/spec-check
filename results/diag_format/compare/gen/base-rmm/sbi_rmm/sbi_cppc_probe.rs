pub open spec fn sbi_cppc_probe_spec(error: long, value: long, cppc_reg_id: uint32_t, old_s: S, new_s: S) -> bool {
    (IsReservedCppcRegId(cppc_reg_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (CppcProbeFailedForUnspecifiedReason(cppc_reg_id) ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> value == CppcRegWidth(cppc_reg_id))
    && (!IsCppcRegImplemented(cppc_reg_id) ==> value == 0)
    && (old_s == new_s)
}