pub open spec fn sbi_sse_write_attrs_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> (old_s.sse_event_id_is_valid(old_s, event_id) == false || !old_s.platform_supports_event(event_id)))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (event_id == 0 || attr_count == 0 || !old_s.event_attribute_values_legal(old_s, event_id, base_attr_id, attr_count)))
    && (result == SBI_SBI_ERR_DENIED ==> (old_s.sse_event_id_is_valid(old_s, event_id) && !old_s.event_attribute_values_writable(old_s, event_id, base_attr_id, attr_count)))
    && (result == SBI_SBI_ERR_INVALID_STATE ==> (old_s.sse_event_id_is_valid(old_s, event_id) && !old_s.event_attribute_values_state_legal(old_s, event_id, base_attr_id, attr_count)))
    && (result == SBI_SBI_ERR_BAD_RANGE ==> (old_s.sse_event_id_is_valid(old_s, event_id) && !old_s.event_attribute_ids_reserved_free(old_s, event_id, base_attr_id, attr_count)))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (input_phys_lo % (XLEN / 8) != 0 || (input_phys_hi as int) < 0 || (input_phys_lo as int) + (XLEN / 8) * attr_count > (input_phys_hi as int)))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (old_s.sse_event_id_is_valid(old_s, event_id) && old_s.platform_supports_event(event_id) && old_s.event_attribute_values_legal(old_s, event_id, base_attr_id, attr_count) && old_s.event_attribute_values_writable(old_s, event_id, base_attr_id, attr_count) && old_s.event_attribute_ids_reserved_free(old_s, event_id, base_attr_id, attr_count) && old_s.event_attribute_values_state_legal(old_s, event_id, base_attr_id, attr_count) && new_s.event_attributes == old_s.event_attributes))
}