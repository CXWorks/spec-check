pub open spec fn powercap_cap_notify__3_10_3_15_spec(result: int32, old_s: S, new_s: S) -> bool {
    (old_s.protocol_message_attributes == 0x0 => ResultEqual(result, NOT_FOUND))
    && (old_s.protocol_message_attributes != 0x0 => ResultEqual(result, SUCCESS))
    && (old_s.domain_id == 0xFFFFFFFF => ResultEqual(result, NOT_FOUND))
    && (old_s.notify_enable & 0xFFFFFFFE != 0 => ResultEqual(result, INVALID_PARAMETERS))
    && (old_s.notify_enable & 0x1 == 0 => ResultEqual(result, SUCCESS))
    && (old_s.notify_enable & 0x1 == 1 => ResultEqual(result, SUCCESS))
}