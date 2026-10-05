pub open spec fn system_power_state_get__3_4_2_6_spec(status: i32, system_state: u32, old_s: S, new_s: S) -> bool {
    (status == 0i32 ==> (
        system_state == 0x0u32
        || system_state == 0x3u32
        || system_state == 0x4u32
        || system_state >= 0x8000_0000u32
    ))
    && (new_s == old_s)
}
