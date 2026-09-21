pub open spec fn ffa_rxtx_map_spec(
    result: FfaReturnCode,
    old_s: FfaState,
    new_s: FfaState,
    tx_addr: UInt64,
    rx_addr: UInt64,
    page_count: UInt32,
) -> bool {
    // Failure: INVALID_PARAMETERS
    // - TX or RX buffer address is not properly aligned
    // - Invalid number of pages specified
    // - TX or RX buffer address is not mapped to a valid memory region in the caller's translation regime
    // - Invalid endpoint ID encoded in the Endpoint RX/TX descriptor
    // - Invalid encoding of the Composite memory region descriptor in the Endpoint RX/TX descriptor
    (!((tx_addr as int) % 4096 == 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!((rx_addr as int) % 4096 == 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!((page_count & 0x3F) > 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsMappedInCallerRegime(old_s, tx_addr) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsMappedInCallerRegime(old_s, rx_addr) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsValidEndpointDescriptor(old_s, tx_addr) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsValidCompositeDescriptor(old_s, tx_addr) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    // Failure: NO_MEMORY
    // - Not enough memory to map the buffers in the translation regime of the callee
    // - Not enough memory in TX buffer of Hypervisor to describe caller buffer pair to SPM
    (!HasEnoughMemory(old_s, tx_addr, rx_addr, page_count) ==> ResultEqual(result, FFA_ERROR_NO_MEMORY))
    // Failure: DENIED
    // - Buffer pair already registered for the FF-A component with specified ID
    // - A VM's buffer pair cannot be registered by the SPMC since no SP sends or receives Indirect messages
    (!IsBufferPairUnregistered(old_s, tx_addr, rx_addr) ==> ResultEqual(result, FFA_ERROR_DENIED))
    // Failure: NOT_SUPPORTED
    // - This function is not implemented at this FF-A instance
    (!IsFunctionSupported(old_s) ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
    // Success: FFA_SUCCESS
    // - Maps the RX/TX buffer pair in the translation regime of the callee
    // - Both Hypervisor and SPM must ensure the caller has exclusive access and ownership of the RX/TX buffer memory regions
    (ResultEqual(result, FFA_SUCCESS) ==> (
        IsMappedInCallerRegime(new_s, tx_addr)
        && IsMappedInCallerRegime(new_s, rx_addr)
        && IsBufferPairRegistered(new_s, tx_addr, rx_addr)
        && (new_s == old_s) // No other state changes implied
    ))
}