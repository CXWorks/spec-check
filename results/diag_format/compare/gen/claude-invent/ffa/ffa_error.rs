pub open spec fn ffa_error_spec(function_id: UInt32, target_info: UInt32, error_code: i32, w3: UInt32, w4: UInt32, w5: UInt32, w6: UInt32, w7: UInt32, smc_conduit_at_ns_virtual_instance: bool) -> bool {
    (function_id == 0x84000060u32 || function_id == 0xC4000060u32)
    && (!smc_conduit_at_ns_virtual_instance ==> target_info == 0)
    && (-10i32 <= error_code && error_code <= -1i32)
    && w3 == 0
    && w4 == 0
    && w5 == 0
    && w6 == 0
    && w7 == 0
}
