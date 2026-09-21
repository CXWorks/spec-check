pub open spec fn 3.12.6.1_telemetry_update_spec(agent_id: UInt32, status: int32, num_dwords: UInt32, array: [UInt32], old_s: S, new_s: S) -> bool {
    (status != 0 ==> agent_id == 0)
    && (status == SCMI_PARTIAL_ERROR ==> num_dwords > 0)
    && (num_dwords % 2 == 0)
    && (array.len() == num_dwords)
}