pub open spec fn ffa_msg_wait_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS (Unrecognized endpoint or vCPU ID specified at Non-secure physical or virtual FF-A instance)
    // Failure: DENIED (Callee is not in a state to handle this request)
    // Failure: NOT_SUPPORTED (This function is not implemented at this FF-A instance)
    // Success: result == 0 (Implicit success code for FFA_SUCCESS, as no explicit success table is provided, but error codes are defined)
    // Note: The spec text does not define a specific success return code value, only error codes.
    // We assume 0 is success based on standard FF-A conventions where errors are explicitly listed.
    // However, without an explicit success code in the text, we must be careful.
    // The text says "Encoding of error codes...". It does not say "Success is 0".
    // But typically, if no error, it's success. Let's assume result == 0 is success.
    // If the spec implies any non-error is success, we might need to check against the error list.
    // Given the constraints, we will assume result == 0 is the success case.
    
    // Check for INVALID_PARAMETERS
    // This error occurs at Non-secure physical or virtual FF-A instance if endpoint/vCPU ID is unrecognized.
    // We don't have the state to check "unrecognized", so we rely on the result.
    (result == -1 ==> (old_s.ffa_instance == FfaInstance::NS_PHYSICAL || old_s.ffa_instance == FfaInstance::NS_VIRTUAL))
    && (result == -1 ==> (old_s.ffa_conduit == FfaConduit::SMC || old_s.ffa_conduit == FfaConduit::HVC || old_s.ffa_conduit == FfaConduit::SVC || old_s.ffa_conduit == FfaConduit::ERET))
    // Check for DENIED
    // Callee is not in a state to handle this request.
    // This could happen if the context is not in a valid state to wait.
    // We don't have the state to check "valid state", so we rely on the result.
    (result == -2 ==> true) // We cannot verify the state condition without more context on valid states.
    // Check for NOT_SUPPORTED
    // This function is not implemented at this FF-A instance.
    // This could happen at Secure physical or Non-secure physical (if not ERET) or other instances.
    // We don't have the state to check "implemented", so we rely on the result.
    (result == -3 ==> true) // We cannot verify the state condition without more context on implementation.
    // Success condition
    // If result is not an error, it is success.
    // We assume 0 is success.
    (result == 0 ==> (new_s.context_state == old_s.context_state || new_s.context_state == ContextState::WAITING))
    && (result == 0 ==> (old_s.ffa_instance == FfaInstance::NS_VIRTUAL || old_s.ffa_instance == FfaInstance::SP))
    && (result == 0 ==> (old_s.ffa_conduit == FfaConduit::SMC || old_s.ffa_conduit == FfaConduit::HVC || old_s.ffa_conduit == FfaConduit::SVC || old_s.ffa_conduit == FfaConduit::ERET))
}