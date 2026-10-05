pub open spec fn performance_describe_levels__3_5_6_8_spec(
    result: int32,
    num_levels: uint32,
    perf_levels: array<array<uint32>, 5>,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    skip_index: uint32,
) -> bool {
    // Failure condition: NOT_FOUND if domain_id does not point to a valid domain
    (result == NOT_FOUND ==> !IsDomainValid(old_s, domain_id))
    // Success condition: result is SUCCESS
    (result == SUCCESS ==> {
        // num_levels must be valid: Bits[31:16] remaining, Bits[15:12] reserved (0), Bits[11:0] count
        let remaining = (num_levels >> 16) as int;
        let reserved = ((num_levels >> 12) & 0xF) as int;
        let count = (num_levels & 0xFFF) as int;
        reserved == 0
            && count >= 0
            && remaining >= 0
            // perf_levels array must have exactly 'count' entries of 5 uint32s each
            && (count == 0 || (perf_levels.len() as int == count * 5))
            // Each entry must be valid:
            // entry[0]: Performance level value (>= 0)
            // entry[1]: Power cost (>= 0, 0 means not reported)
            // entry[2]: Attributes (Bits[31:16] == 0, Bits[15:0] == transition latency)
            // entry[3]: Indicative Frequency (>= 0, 0 means not reported)
            // entry[4]: Level Index (>= 0)
            && (count == 0 || {
                for i in 0..count {
                    let base = i * 5;
                    let entry0 = perf_levels[base];
                    let entry1 = perf_levels[base + 1];
                    let entry2 = perf_levels[base + 2];
                    let entry3 = perf_levels[base + 3];
                    let entry4 = perf_levels[base + 4];
                    entry0 >= 0
                        && entry1 >= 0
                        && ((entry2 >> 16) as int == 0)
                        && ((entry2 & 0xFFFF) as int) >= 0
                        && entry3 >= 0
                        && entry4 >= 0
                }
            })
            // Entries must be in numerically ascending order of performance level values (entry[0])
            && (count == 0 || {
                for i in 0..count - 1 {
                    let base = i * 5;
                    let next_base = (i + 1) * 5;
                    perf_levels[base] < perf_levels[next_base]
                }
            })
    })
}