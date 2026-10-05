pub open spec fn clock_rate_get__3_6_2_8_spec(status: Int32, rate: Seq<UInt32>, clock_id: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> (status == NOT_FOUND && new_s == old_s))
    && (ClockExists(old_s, clock_id) ==> (
        status == SUCCESS
        && rate.len() == 2
        && (rate[0] as int) == (ClockRate(old_s, clock_id) as int) % 0x1_0000_0000
        && (rate[1] as int) == (ClockRate(old_s, clock_id) as int) / 0x1_0000_0000
        && new_s == old_s
    ))
}
