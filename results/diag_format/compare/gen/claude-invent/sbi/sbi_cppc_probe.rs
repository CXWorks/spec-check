pub open spec fn sbi_cppc_probe_spec(cppc_reg_id: UInt32, error: SbiErrorCode, value: UInt64, old_s: S, new_s: S) -> bool {
    (CppcRegIdIsReserved(cppc_reg_id) ==> error == SBI_ERR_INVALID_PARAM)
    && (!CppcRegIdIsReserved(cppc_reg_id) ==> (error == SBI_SUCCESS || error == SBI_ERR_FAILED))
    && ((!CppcRegIdIsReserved(cppc_reg_id) && error == SBI_SUCCESS) ==> (
        (CppcRegIsImplemented(old_s, cppc_reg_id) ==> value == CppcRegWidth(old_s, cppc_reg_id))
        && (!CppcRegIsImplemented(old_s, cppc_reg_id) ==> value == 0)
    ))
    && new_s == old_s
}
