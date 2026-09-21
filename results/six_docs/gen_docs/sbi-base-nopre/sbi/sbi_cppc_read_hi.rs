pub open spec fn sbi_cppc_read_hi_spec(result: SbiRet, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> cppc_reg_id_is_reserved(old_s, result.cppc_reg_id))
    && (result.error == SBI_ERR_NOT_SUPPORTED ==> cppc_reg_id_is_not_implemented(old_s, result.cppc_reg_id))
    && (result.error == SBI_ERR_DENIED ==> cppc_reg_id_is_write_only(old_s, result.cppc_reg_id))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (old_s.supervisor_mode_xlen >= 64 ==> result.value == 0))
    && (result.error == SBI_SUCCESS ==> (old_s.supervisor_mode_xlen < 64 ==> result.value == old_s.cppc_register_high_bits(result.cppc_reg_id)))
    && (result.error == SBI_SUCCESS ==> new_s == old_s)
}