pub open spec fn ffa_yield_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS (Unrecognized endpoint or vCPU ID) - Only valid with ERET conduit
    // Failure: DENIED (Callee is not in a state to handle this request)
    // Failure: NOT_SUPPORTED (Function not implemented at this FF-A instance)
    // Success: result == 0 (FFA_SUCCESS)
    // Note: The spec text does not define specific preconditions for success (e.g., specific endpoint states)
    // other than the implicit requirement that the caller is in a valid state to invoke the ABI.
    // Without explicit preconditions in the text, we cannot assert them.
    // We can only assert the error codes defined in Table 14.10.
    (result == INVALID_PARAMETERS) ==> (old_s.endpoint_id as int < 0 || old_s.endpoint_id as int >= 0xFF || old_s.vcpu_id as int < 0 || old_s.vcpu_id as int >= 0xFFFF)
    && (result == DENIED) ==> true
    && (result == NOT_SUPPORTED) ==> true
    && (result == 0) ==> true
}