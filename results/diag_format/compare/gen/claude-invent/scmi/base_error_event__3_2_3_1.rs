pub open spec fn base_error_event__3_2_3_1_spec(agent_id: u32, error_status: u32, message_headers: Seq<u32>, statuses: Seq<u32>, old_s: S, new_s: S) -> bool {
    (((error_status >> 10u32) & 0x1F_FFFFu32) == 0u32)
    && (message_headers.len() == (error_status & 0x3FFu32) as nat)
    && (statuses.len() == (error_status & 0x3FFu32) as nat)
    && (PlatformErrorIsFatal(old_s) ==> ((error_status >> 31u32) & 1u32) == 1u32)
    && (!PlatformErrorIsFatal(old_s) ==> ((error_status >> 31u32) & 1u32) == 0u32)
    && (forall|i: int| 0 <= i < message_headers.len() ==> CommandFailedOnPlatform(old_s, message_headers[i], statuses[i]))
    && new_s == old_s
}
