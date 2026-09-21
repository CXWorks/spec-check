pub open spec fn sbi_sse_read_attrs_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.event_id as int < 0 || old_s.attr_count as int == 0))
    && (result == SBI_SBI_ERR_BAD_RANGE ==> (exists i: int, 0 <= i && i < old_s.attr_count as int && (old_s.base_attr_id as int + i) as int < 0 || (old_s.base_attr_id as int + i) as int >= old_s.event_id as int))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (old_s.output_phys_lo as int % (old_s.xlen as int / 8) != 0))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> (true))
    && (result == SBI_SBI_ERR_FAILED ==> (true))
    && (result == SBI_SBI_SUCCESS ==> (old_s.event_id as int >= 0 && old_s.attr_count as int > 0 && old_s.base_attr_id as int >= 0 && old_s.output_phys_lo as int % (old_s.xlen as int / 8) == 0))
}