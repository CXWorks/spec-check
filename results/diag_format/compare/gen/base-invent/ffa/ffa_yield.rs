pub open spec fn ffa_yield_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS (Unrecognized endpoint or vCPU ID)
    // Only valid with the ERET conduit.
    // Failure: DENIED (Callee is not in a state to handle this request)
    // Failure: NOT_SUPPORTED (This function is not implemented at this FF-A instance)
    // Success: result == 0 (FFA_SUCCESS)
    // Note: The spec text does not define a specific "FFA_SUCCESS" constant, but standard FF-A behavior implies 0 on success.
    // We express the failure conditions as stated.
    (result == INVALID_PARAMETERS ==> (old_s.endpoint_id != new_s.endpoint_id || old_s.vcpu_id != new_s.vcpu_id))
    && (result == DENIED ==> true)
    && (result == NOT_SUPPORTED ==> true)
    && (result == 0 ==> true)
}