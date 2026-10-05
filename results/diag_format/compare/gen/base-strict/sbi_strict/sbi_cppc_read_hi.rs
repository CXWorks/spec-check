pub open spec fn sbi_cppc_read_hi_spec(error: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
    (IsReservedCppcReg(old_s, cppc_reg_id(old_s)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsImplementedCppcReg(old_s, cppc_reg_id(old_s)) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
    && (IsWriteOnlyCppcReg(old_s, cppc_reg_id(old_s)) ==> ResultEqual(error, SBI_ERR_DENIED))
    && (CppcReadRequestFailed(old_s, cppc_reg_id(old_s)) ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> (SupervisorXlen(old_s) >= 64 ==> value == 0))
    && (ResultEqual(error, SBI_SUCCESS) ==> (SupervisorXlen(old_s) < 64 ==> value == Bits(CppcRegValue(old_s, cppc_reg_id(old_s)), 63, 32)))
}