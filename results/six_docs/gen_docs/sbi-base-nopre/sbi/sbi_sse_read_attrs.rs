pub open spec fn sbi_sse_read_attrs_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_INVALID_PARAM ==> (event_id(old_s) != 0 || attr_count(old_s) == 0))
    && (result == SBI_ERR_BAD_RANGE ==> exists(i: u32, i < attr_count(old_s) && is_reserved_event_attr_id(old_s, base_attr_id(old_s) + i)))
    && (result == SBI_ERR_INVALID_ADDRESS ==> !is_aligned(output_phys_lo(old_s), XLEN / 8))
    && (result == SBI_ERR_NOT_SUPPORTED ==> !platform_supports_event_id(old_s, event_id(old_s)))
    && (result == SBI_ERR_FAILED ==> true)
    && (result == SBI_SUCCESS ==> (is_aligned(output_phys_lo(old_s), XLEN / 8) && attr_count(old_s) > 0 && !exists(i: u32, i < attr_count(old_s) && is_reserved_event_attr_id(old_s, base_attr_id(old_s) + i))))
}