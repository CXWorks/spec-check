pub open spec fn ffa_msg_wait_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS (0x80000001)
    // - Unrecognized endpoint or vCPU ID specified at Non-secure physical or virtual FF-A instance.
    //   (Implies: Instance is NS Physical or NS Virtual AND Endpoint/vCPU ID is invalid/unrecognized)
    // Failure: DENIED (0x80000002)
    // - Callee is not in a state to handle this request.
    //   (Implies: Instance is Secure Physical OR Secure Virtual AND Callee is not in a state to handle this request)
    // Failure: NOT_SUPPORTED (0x80000003)
    // - This function is not implemented at this FF-A instance.
    //   (Implies: Instance is Secure Physical OR Secure Virtual AND Function is not implemented)
    // Note: The spec text does not define specific failure conditions for NS Physical (ERET) or NS Virtual (SMC/HVC/ERET)
    // other than INVALID_PARAMETERS for NS Physical/NS Virtual regarding endpoint/vCPU ID.
    // It also does not define success conditions or state transitions in terms of verifiable predicates for the spec.
    // Therefore, we return true (unconstrained) as per the "Fully unconstrained specs rule" for commands where
    // the provided text does not state explicit failure/success conditions that can be formalized.
    true
}