pub open spec fn protocol_attributes__3_5_6_3_spec(result: int32, attributes: uint32, statistics_address_low: uint32, statistics_address_high: uint32, statistics_len: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes & 0xFFFF == 0)
    && (statistics_address_low & 0xFFFF_FFFF == 0)
    && (statistics_address_high & 0xFFFF_FFFF == 0)
    && (statistics_len == 0 || (statistics_address_low as int) >= 0)
    && (statistics_len == 0 || (statistics_address_high as int) >= 0)
    && (statistics_len == 0 || (statistics_address_low as int) + (statistics_len as int) <= (statistics_address_high as int) * 0x100000000 + (statistics_address_high as int))
    && (old_s == new_s)
}