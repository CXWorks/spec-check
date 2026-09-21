pub open spec fn sbi_pmu_counter_stop_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (stop_flags_reserved(old_s, stop_flags) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (counter_invalid(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (counter_already_stopped(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
    && (SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT(stop_flags) && !shmem_available(old_s) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (result.is_Ok() ==> (counter_stopped(new_s, counter_idx_base, counter_idx_mask) && !counter_stopped(old_s, counter_idx_base, counter_idx_mask)))
    && (result.is_Ok() ==> (SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT(stop_flags) ==> snapshot_saved(old_s, new_s, counter_idx_base, counter_idx_mask)))
    && (result.is_Ok() ==> (SBI_PMU_STOP_FLAG_RESET(stop_flags) ==> counter_reset(new_s, counter_idx_base, counter_idx_mask)))
    && (result.is_Ok() ==> (SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT(stop_flags) ==> overflown_bitmap_updated(old_s, new_s)))
    && (result.is_Ok() ==> (SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT(stop_flags) ==> overflown_bitmap_updated(old_s, new_s)))
}

fn stop_flags_reserved(old_s: S, stop_flags: u64) -> bool {
    (stop_flags & !SBI_PMU_STOP_FLAG_RESET | SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT) != 0
}

fn counter_invalid(old_s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool {
    (counter_idx_base & counter_idx_mask) != 0
}

fn counter_already_stopped(old_s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool {
    (counter_idx_base & counter_idx_mask) != 0
}

fn shmem_available(old_s: S) -> bool {
    old_s.shmem != null
}

fn counter_stopped(new_s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool {
    (new_s.counters & counter_idx_mask) == counter_idx_mask
}

fn snapshot_saved(old_s: S, new_s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool {
    (new_s.shmem != null) && (new_s.shmem_snapshot == old_s.shmem_snapshot)
}

fn counter_reset(new_s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool {
    (new_s.counters & counter_idx_mask) == 0
}

fn overflown_bitmap_updated(old_s: S, new_s: S) -> bool {
    (new_s.overflown_bitmap == old_s.overflown_bitmap)
}