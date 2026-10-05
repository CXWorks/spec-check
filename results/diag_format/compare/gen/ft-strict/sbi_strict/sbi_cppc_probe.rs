pub open spec fn sbi_cppc_probe_spec(cppc_reg_id: uint32_t, result: sbiret, value: int, old_s: S, new_s: S) -> bool {
  (IsReservedCppcRegId(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (CppcProbeFailed(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (ResultEqual(result, SBI_SUCCESS) ==> IsCppcRegImplemented(old_s, cppc_reg_id) ==> value == CppcRegWidth(old_s, cppc_reg_id))
  && (ResultEqual(result, SBI_SUCCESS) ==> !IsCppcRegImplemented(old_s, cppc_reg_id) ==> value == 0)
  && ((!IsReservedCppcRegId(old_s, cppc_reg_id) &&
       !CppcProbeFailed(old_s, cppc_reg_id))
    ==> ResultEqual(result, SBI_SUCCESS))
}