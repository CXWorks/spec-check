pub open spec fn 3.5.6.8_performance_describe_levels_spec(
    result: int32,
    domain_id: uint32,
    skip_index: uint32,
    num_levels: uint32,
    perf_levels: array<array<uint32>, 5>,
    old_s: S,
    new_s: S
) -> bool {
    (result == 0 ==> (num_levels >= 0 && num_levels <= 4095))
    && (result == 0 ==> (skip_index >= 0))
    && (result == 0 ==> (perf_levels.len() == num_levels * 5))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels * 5 | perf_levels[i] >= 0))
    && (result != 0 ==> (num_levels == 0))
    && (result != 0 ==> (perf_levels.len() == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        perf_levels[i * 5 + 0] >= 0 &&
        perf_levels[i * 5 + 1] >= 0 &&
        perf_levels[i * 5 + 2] >= 0 &&
        perf_levels[i * 5 + 3] >= 0 &&
        perf_levels[i * 5 + 4] >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels - 1 |
        perf_levels[i * 5 + 0] < perf_levels[(i + 1) * 5 + 0]))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF) <= 0xFFFF))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) <= 0xFFFFFFFF))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) <= 0xFFFFFFFF))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 0] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 3] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 4] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 2] & 0xFFFF0000) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |
        (perf_levels[i * 5 + 1] & 0xFFFFFFFF) >= 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < num_levels |