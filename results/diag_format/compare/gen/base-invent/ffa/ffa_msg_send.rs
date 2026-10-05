pub open spec fn ffa_msg_send_spec(result: u32, old_s: S, new_s: S) -> bool {
    // Failure condition: Must not be invoked when the caller is processing a Direct request.
    // (The spec implies this is a precondition; if violated, the result must be an error code, not FFA_SUCCESS)
    (!IsProcessingDirectRequest(old_s) ==> ResultIsError(result))
    // Success condition: w0 contains FFA_SUCCESS function ID.
    // w1/x1-w7/x7 are Reserved (MBZ).
    // Successful completion does not imply the message has been read.
    (IsProcessingDirectRequest(old_s) ==> (result == FFA_SUCCESS && w1_is_mbz(new_s) && w2_is_mbz(new_s) && w3_is_mbz(new_s) && w4_is_mbz(new_s) && w5_is_mbz(new_s) && w6_is_mbz(new_s) && w7_is_mbz(new_s)))
}

// Helper predicates (uninterpreted as they are not defined in the provided context snippet)
pub open spec fn IsProcessingDirectRequest(s: S) -> bool { ... }
pub open spec fn ResultIsError(r: u32) -> bool { ... }
pub open spec fn w1_is_mbz(s: S) -> bool { ... }
pub open spec fn w2_is_mbz(s: S) -> bool { ... }
pub open spec fn w3_is_mbz(s: S) -> bool { ... }
pub open spec fn w4_is_mbz(s: S) -> bool { ... }
pub open spec fn w5_is_mbz(s: S) -> bool { ... }
pub open spec fn w6_is_mbz(s: S) -> bool { ... }
pub open spec fn w7_is_mbz(s: S) -> bool { ... }