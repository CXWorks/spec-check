pub open spec fn ffa_rxtx_unmap_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_SUCCESS)
    && (result == FFA_ERROR_INVALID_PARAMETERS ==> !IsBufferPairRegistered(old_s))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> !IsFfaRxtxUnmapSupported(old_s))
}

pub open spec fn IsBufferPairRegistered(s: S) -> bool {
    // Placeholder for the actual predicate checking if a buffer pair is registered
    // The exact implementation depends on the specific state structure S
    true
}

pub open spec fn IsFfaRxtxUnmapSupported(s: S) -> bool {
    // Placeholder for the actual predicate checking if the function is supported
    // The exact implementation depends on the specific state structure S
    true
}