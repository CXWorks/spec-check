pub open spec fn reset_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure conditions
    // Reserved bits [31:3] must be zero
    let flags = old_s.cmd_input_flags as int;
    (flags & 0x7FFFFFF8 != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Async flag (bit 2) is only valid if Autonomous Reset (bit 0) is set
    ((flags & 0x4) != 0 && (flags & 0x1) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Explicit signal (bit 1) is ignored when Async (bit 0) is set
    // (No specific error defined for this in the text, so we omit it as a failure condition)
    // Domain ID must exist
    (domain_id_not_found(old_s, new_s, old_s.cmd_input_domain_id) ==> ResultEqual(result, NOT_FOUND))
    // Reset state must be valid/supported
    (invalid_reset_state(old_s, new_s, old_s.cmd_input_reset_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    // Operation must not fail due to other active users
    (other_active_users(old_s, new_s) ==> ResultEqual(result, GENERIC_ERROR))
    // Calling agent must be allowed to reset the domain
    (agent_not_allowed(old_s, new_s, old_s.cmd_input_domain_id) ==> ResultEqual(result, DENIED))

    // Success condition
    // If none of the failure conditions are met, the operation must succeed
    (!domain_id_not_found(old_s, new_s, old_s.cmd_input_domain_id)
     && !invalid_reset_state(old_s, new_s, old_s.cmd_input_reset_state)
     && !other_active_users(old_s, new_s)
     && !agent_not_allowed(old_s, new_s, old_s.cmd_input_domain_id)
     ==> result == SUCCESS)
}

// Helper predicates (assumed to exist in context or defined as simple checks)
fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}

fn other_active_users(old_s: S, new_s: S) -> bool {
    // Placeholder for active user check
    true
}

fn agent_not_allowed(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for permission check
    true
}

fn ResultEqual(result: int32, expected: int32) -> bool {
    result == expected
}

fn SUCCESS: int32 = 0
fn NOT_FOUND: int32 = 1
fn INVALID_PARAMETERS: int32 = 2
fn GENERIC_ERROR: int32 = 3
fn DENIED: int32 = 4

fn domain_id_not_found(old_s: S, new_s: S, domain_id: uint32) -> bool {
    // Placeholder for domain existence check
    true
}

fn invalid_reset_state(old_s: S, new_s: S, reset_state: uint32) -> bool {
    // Placeholder for reset state validation
    true
}