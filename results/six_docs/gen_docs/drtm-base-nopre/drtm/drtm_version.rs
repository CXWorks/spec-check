pub open spec fn drtm_version_spec(result: u32, old_s: S, new_s: S) -> bool {
    (result == 0xC400_0110)
}