pub open spec fn sbi_cppc_read_spec(cppc_reg_id: UInt32, ret_value: UInt64, old_s: S, new_s: S) -> bool {
    (SupervisorXlen(old_s) == 32 ==> ret_value == (CppcRegisterValue(old_s, cppc_reg_id) & 0xFFFF_FFFFu64))
    && (SupervisorXlen(old_s) != 32 ==> ret_value == CppcRegisterValue(old_s, cppc_reg_id))
    && new_s == old_s
}
