pub open spec fn sbi_cppc_write_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> cppc_reg_id_is_reserved(old_s, result))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> cppc_reg_id_not_implemented(old_s, result))
    && (result == SBI_SBI_SUCCESS ==> cppc_reg_id_is_valid(old_s, result) && cppc_reg_id_is_implemented(old_s, result) && new_s.cppc_registers[old_s.cppc_reg_id] == old_s.val)
}