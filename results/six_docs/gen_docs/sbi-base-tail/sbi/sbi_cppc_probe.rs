pub open spec fn sbi_cppc_probe_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.cmd_input_cppc_reg_id as int) == 0)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (new_s.cmd_output_value as int) > 0)
    && (result == 0 ==> (new_s.cmd_output_value as int) == 0)
}