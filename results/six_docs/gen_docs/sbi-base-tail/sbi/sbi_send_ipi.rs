pub open spec fn sbi_send_ipi_spec(result: int, hart_mask: UInt64, hart_mask_base: UInt64, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (hart_mask == 0 || hart_mask_base == 0 || (hart_mask as int) < 0 || (hart_mask_base as int) < 0))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
    && (result != SBI_SBI_SUCCESS ==> true)
    && (result != SBI_SBI_ERR_INVALID_PARAM ==> true)
    && (result != SBI_SBI_ERR_FAILED ==> true)
}