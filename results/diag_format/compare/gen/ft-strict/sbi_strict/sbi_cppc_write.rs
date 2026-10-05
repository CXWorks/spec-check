pub open spec fn sbi_cppc_write_spec(cppc_reg_id: UInt32, val: UInt64, result: SbiError, old_s: S, new_s: S) -> bool {
  (CppcRegIsReserved(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!CppcRegIsImplemented(old_s, cppc_reg_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> CppcRegValue(new_s, cppc_reg_id) == val)
  && ((!CppcRegIsReserved(old_s, cppc_reg_id) &&
       CppcRegIsImplemented(old_s, cppc_reg_id))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> CppcRegValue(new_s, cppc_reg_id) == CppcRegValue(old_s, cppc_reg_id))
}