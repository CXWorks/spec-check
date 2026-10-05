pub open spec fn sbi_cppc_write_spec(cppc_reg_id: UInt32, val: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (CppcRegIdIsReserved(cppc_reg_id) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((!CppcRegIdIsReserved(cppc_reg_id) && !CppcRegIsImplemented(old_s, cppc_reg_id)) ==> (result.error == SBI_ERR_NOT_SUPPORTED && new_s == old_s))
    && ((!CppcRegIdIsReserved(cppc_reg_id) && CppcRegIsImplemented(old_s, cppc_reg_id)) ==> (result.error == SBI_SUCCESS && CppcRegWritten(old_s, new_s, cppc_reg_id, val)))
}
