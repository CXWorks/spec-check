pub open spec fn sbi_cppc_probe_spec(cppc_reg_id: uint32_t, error: long, value: long, old_s: S, new_s: S) -> bool {
  (IsReservedCppcRegId(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (CppcProbeFailedForUnspecifiedReason(old_s, cppc_reg_id) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> value == CppcRegWidth(cppc_reg_id))
  && (!IsCppcRegImplemented(old_s, cppc_reg_id) ==> value == 0)
  && ((!IsReservedCppcRegId(old_s, cppc_reg_id) &&
       !CppcProbeFailedForUnspecifiedReason(old_s, cppc_reg_id))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> value == 0)
}