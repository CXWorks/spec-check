pub open spec fn sbi_cppc_write_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> cppc_reg_id_is_reserved(old_s, result.cppc_reg_id))
    && (result.error == SBI_ERR_NOT_SUPPORTED ==> cppc_reg_id_not_implemented(old_s, result.cppc_reg_id))
    && (result.error == SBI_SUCCESS ==> cppc_reg_id_not_reserved(old_s, result.cppc_reg_id) && cppc_reg_id_implemented(old_s, result.cppc_reg_id) && new_s.cppc_registers[result.cppc_reg_id] == old_s.val)
}