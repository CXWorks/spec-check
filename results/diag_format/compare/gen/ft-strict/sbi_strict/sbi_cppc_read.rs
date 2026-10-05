pub open spec fn sbi_cppc_read_spec(cppc_reg_id: UInt32, value: UInt64, old_s: S, new_s: S) -> bool {
  (SupervisorXlen(old_s) != 32 ==> value == CppcRegisterValue(new_s, cppc_reg_id))
  && (SupervisorXlen(old_s) == 32 ==> value == Bits(CppcRegisterValue(new_s, cppc_reg_id), 31, 0))
  && ((!(SupervisorXlen(old_s) != 32)) ==> true)
}