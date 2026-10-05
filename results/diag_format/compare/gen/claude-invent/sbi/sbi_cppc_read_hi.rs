pub open spec fn sbi_cppc_read_hi_spec(cppc_reg_id: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
    (CppcRegIsReserved(old_s, cppc_reg_id) ==> result.error == SBI_ERR_INVALID_PARAM)
    && ((!CppcRegIsReserved(old_s, cppc_reg_id) && !CppcRegIsImplemented(old_s, cppc_reg_id)) ==> result.error == SBI_ERR_NOT_SUPPORTED)
    && ((!CppcRegIsReserved(old_s, cppc_reg_id) && CppcRegIsImplemented(old_s, cppc_reg_id) && CppcRegIsWriteOnly(old_s, cppc_reg_id)) ==> result.error == SBI_ERR_DENIED)
    && ((!CppcRegIsReserved(old_s, cppc_reg_id) && CppcRegIsImplemented(old_s, cppc_reg_id) && !CppcRegIsWriteOnly(old_s, cppc_reg_id)) ==> (result.error == SBI_SUCCESS || result.error == SBI_ERR_FAILED))
    && ((result.error == SBI_SUCCESS && (SupervisorXlen(old_s) as int) >= 64) ==> result.value == 0)
    && ((result.error == SBI_SUCCESS && (SupervisorXlen(old_s) as int) < 64) ==> (result.value as int) == ((CppcRegValue(old_s, cppc_reg_id) as int) / 0x1_0000_0000) % 0x1_0000_0000)
    && (new_s == old_s)
}
