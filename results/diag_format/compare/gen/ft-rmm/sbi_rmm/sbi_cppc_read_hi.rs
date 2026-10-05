pub open spec fn sbi_cppc_read_hi_spec(cppc_reg_id: UInt32, result: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
  (IsReservedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsImplementedCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (IsWriteOnlyCppcReg(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (CppcRegReadFailed(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (ResultEqual(result, SBI_SUCCESS) ==> SupervisorXlen(old_s) < 64 ==> value == CppcRegValue(old_s, cppc_reg_id)[63:32])
  && (ResultEqual(result, SBI_SUCCESS) ==> SupervisorXlen(old_s) >= 64 ==> value == 0)
  && ((!(IsReservedCppcReg(old_s, cppc_reg_id)) &&
       IsImplementedCppcReg(old_s, cppc_reg_id) &&
       !(IsWriteOnlyCppcReg(old_s, cppc_reg_id)) &&
       !(CppcRegReadFailed(old_s, cppc_reg_id)))
    ==> ResultEqual(result, SBI_SUCCESS))
}