pub open spec fn sbi_sse_read_attrs_spec(result: SbiError, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, output_phys_lo: UInt64, output_phys_hi: UInt64, old_s: S, new_s: S) -> bool {
    ((!SseEventIdValid(old_s, event_id) || attr_count == 0)
        ==> (result == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((SseEventIdValid(old_s, event_id)
            && attr_count != 0
            && !SseEventSupported(old_s, event_id))
        ==> (result == SBI_ERR_NOT_SUPPORTED && new_s == old_s))
    && ((SseEventIdValid(old_s, event_id)
            && attr_count != 0
            && SseEventSupported(old_s, event_id)
            && (exists|i: int| 0 <= i < (attr_count as int) && SseAttrIdReserved(old_s, (base_attr_id as int) + i)))
        ==> (result == SBI_ERR_BAD_RANGE && new_s == old_s))
    && ((SseEventIdValid(old_s, event_id)
            && attr_count != 0
            && SseEventSupported(old_s, event_id)
            && (forall|i: int| 0 <= i < (attr_count as int) ==> !SseAttrIdReserved(old_s, (base_attr_id as int) + i))
            && ((output_phys_lo as int) % 8 != 0
                || !SseSharedMemValid(old_s, output_phys_lo, output_phys_hi, 8 * (attr_count as int))))
        ==> (result == SBI_ERR_INVALID_ADDRESS && new_s == old_s))
    && ((SseEventIdValid(old_s, event_id)
            && attr_count != 0
            && SseEventSupported(old_s, event_id)
            && (forall|i: int| 0 <= i < (attr_count as int) ==> !SseAttrIdReserved(old_s, (base_attr_id as int) + i))
            && (output_phys_lo as int) % 8 == 0
            && SseSharedMemValid(old_s, output_phys_lo, output_phys_hi, 8 * (attr_count as int)))
        ==> ((result == SBI_SUCCESS || result == SBI_ERR_FAILED)
            && SseEventStateUnchanged(old_s, new_s)
            && (result == SBI_SUCCESS ==>
                (forall|i: int| 0 <= i < (attr_count as int) ==>
                    SharedMemRead64(new_s, output_phys_lo, output_phys_hi, 8 * ((base_attr_id as int) + i))
                        == SseEventAttrValue(old_s, event_id, (base_attr_id as int) + i)))))
}
