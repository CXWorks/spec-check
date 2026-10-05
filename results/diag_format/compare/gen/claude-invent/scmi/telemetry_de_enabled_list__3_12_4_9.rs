pub open spec fn telemetry_de_enabled_list__3_12_4_9_spec(index: u32, flags: u32, status: i32, ret_flags: u32, array: Seq<(u32, u32)>, old_s: S, new_s: S) -> bool {
    ((index as int) >= TelemetryEnabledElementCount(old_s, (flags & 1u32) as int) ==> status == OUT_OF_RANGE)
    && ((index as int) < TelemetryEnabledElementCount(old_s, (flags & 1u32) as int) ==> (
        status == SUCCESS
        && array.len() == ((ret_flags & 0xFFFFu32) as int)
        && (index as int) + ((ret_flags & 0xFFFFu32) as int) + ((ret_flags >> 16u32) as int) == TelemetryEnabledElementCount(old_s, (flags & 1u32) as int)
        && (forall|i: int| 0 <= i < array.len() ==> (
            array[i].0 == TelemetryEnabledElementId(old_s, (flags & 1u32) as int, (index as int) + i)
            && (array[i].1 & 0xFFFF_FFFCu32) == 0u32
            && ((array[i].1 & 3u32) == 1u32 || (array[i].1 & 3u32) == 2u32)
            && (array[i].1 & 3u32) == TelemetryEnabledElementMode(old_s, (flags & 1u32) as int, (index as int) + i)
        ))
    ))
    && new_s == old_s
}
