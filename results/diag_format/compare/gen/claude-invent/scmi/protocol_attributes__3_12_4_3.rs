pub open spec fn protocol_attributes__3_12_4_3_spec(status: i32, de_num: u32, groups_num: u32, de_implementation_rev_dword0: u32, de_implementation_rev_dword1: u32, de_implementation_rev_dword2: u32, de_implementation_rev_dword3: u32, attributes_1: u32, default_blk_ts_rate: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> ((attributes_1 >> 19u32) & 0x7FFu32) == 0u32)
    && (new_s == old_s)
}
