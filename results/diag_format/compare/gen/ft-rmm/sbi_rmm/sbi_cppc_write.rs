pub open spec fn sbi_cppc_write_spec(cppc_reg_id: UInt32, val: UInt64, result: sbiret, old_s: S, new_s: S) -> bool {
  (IsReservedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsImplementedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> CppcReg(new_s, cppc_reg_id) == val)
  && ((!IsReservedCppcReg(old_s, cppc_reg_id) &&
       IsImplementedCppcReg(old_s, cppc_reg_id))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> CppcReg(new_s, cppc_reg_id) == CppcReg(old_s, cppc_reg_id))
}