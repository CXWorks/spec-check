pub open spec fn 3.12.4.8_telemetry_de_configure_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags & 0x3 != 0 || (old_s.flags & 0x4 != 0 && (old_s.flags & 0x3 != 0))))
    && (result == SCMI_IN_USE ==> (old_s.identifier_is_in_use(old_s.identifier, old_s.flags)))
    && (result == SCMI_OUT_OF_RANGE ==> (old_s.de_count + (if old_s.flags & 0x4 != 0 then 1 else 0) > old_s.max_de_count))
    && (result == SCMI_SUCCESS ==> (new_s.de_count == old_s.de_count + (if old_s.flags & 0x4 != 0 then 1 else 0) - (if old_s.flags & 0x4 != 0 && old_s.flags & 0x1 != 0 then 1 else 0)))
    && (result == SCMI_SUCCESS ==> (new_s.flags == old_s.flags))
    && (result == SCMI_SUCCESS ==> (new_s.identifier == old_s.identifier))
    && (result == SCMI_SUCCESS ==> (new_s.shmti_id == old_s.shmti_id))
    && (result == SCMI_SUCCESS ==> (new_s.shmti_de_offset == old_s.shmti_de_offset))
    && (result == SCMI_SUCCESS ==> (new_s.blk_ts_offset == old_s.blk_ts_offset))
}