pub open spec fn sbi_remote_fence_i_spec(result: int, hart_mask: UInt64, hart_mask_base: UInt64, old_s: S, new_s: S) -> bool {
    (hart_mask == 0 || hart_mask_base == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hart_mask != 0 && hart_mask_base != 0 ==> ResultEqual(result, SBI_SBI_SUCCESS))
    && (ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM) ==> hart_mask == 0 || hart_mask_base == 0)
    && (ResultEqual(result, SBI_SBI_SUCCESS) ==> hart_mask != 0 && hart_mask_base != 0)
}

fn ResultEqual(result: int, expected: int) -> bool {
    result == expected
}