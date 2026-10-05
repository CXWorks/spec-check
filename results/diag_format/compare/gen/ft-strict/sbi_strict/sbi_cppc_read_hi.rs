pub open spec fn sbi_cppc_read_hi_spec(cppc_reg_id: UInt32, error: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
  (IsReservedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!IsImplementedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
  && (IsWriteOnlyCppcReg(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_DENIED))
  && (CppcReadRequestFailed(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS))
  && (SupervisorXlen(old_s) >= 64 ==> value == 0)
  && (SupervisorXlen(old_s) < 64 ==> value == Bits(CppcRegValue(new_s, cppc_reg_id), 63, 32))
  && ((!IsReservedCppcReg(old_s, cppc_reg_id) &&
       IsImplementedCppcReg(old_s, cppc_reg_id) &&
       !IsWriteOnlyCppcReg(old_s, cppc_reg_id) &&
       !CppcReadRequestFailed(old_s, cppc_reg_id))
    ==> ResultEqual(error, SBI_SUCCESS))
}