pub open spec fn telemetry_update__3_12_6_1_spec(agent_id: u32, status: i32, num_dwords: u32, array: Seq<u32>, old_s: S, new_s: S) -> bool {
    (agent_id == 0)
    && ((num_dwords as int) % 2 == 0)
    && (array.len() == num_dwords as int)
    && (AllEnabledDesCollectedViaShmtiOrFastChannels(old_s) ==> num_dwords == 0)
}
