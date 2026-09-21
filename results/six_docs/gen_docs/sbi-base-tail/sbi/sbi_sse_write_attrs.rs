pub open spec fn sbi_sse_write_attrs_spec(result: int, old_s: S, new_s: S, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: Address, input_phys_hi: Address) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> event_id != 0)
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (event_id == 0 || attr_count == 0))
    && (result == SBI_SBI_ERR_DENIED ==> true)
    && (result == SBI_SBI_ERR_INVALID_STATE ==> true)
    && (result == SBI_SBI_ERR_BAD_RANGE ==> true)
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (input_phys_lo as int) % (64 / 8) != 0)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
}