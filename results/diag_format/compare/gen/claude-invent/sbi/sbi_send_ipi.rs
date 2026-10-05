pub open spec fn sbi_send_ipi_spec(hart_mask: UInt64, hart_mask_base: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!AllTargetHartsValid(old_s, hart_mask, hart_mask_base) ==> result.error == SBI_ERR_INVALID_PARAM)
    && (AllTargetHartsValid(old_s, hart_mask, hart_mask_base) ==> (result.error == SBI_SUCCESS || result.error == SBI_ERR_FAILED))
    && (result.error == SBI_SUCCESS ==> (
        AllTargetHartsValid(old_s, hart_mask, hart_mask_base)
        && SupervisorSoftwareInterruptPendingOnAllTargetHarts(old_s, new_s, hart_mask, hart_mask_base)
    ))
}
