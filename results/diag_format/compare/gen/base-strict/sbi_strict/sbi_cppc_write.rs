pub open spec fn sbi_cppc_write_spec(result: SbiError, old_s: S, new_s: S, cppc_reg_id: UInt32, val: UInt64) -> bool {
    (CppcRegIsReserved(cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!CppcRegIsImplemented(cppc_reg_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (ResultEqual(result, SBI_SUCCESS) ==> CppcRegValue(cppc_reg_id) == val)
}