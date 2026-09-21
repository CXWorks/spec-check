pub open spec fn ffa_features_spec(result: int, old_s: S, new_s: S) -> bool {
    // Failure condition: if the queried interface or feature is not implemented or invalid,
    // the callee returns FFA_NOT_SUPPORTED.
    // The spec text does not define specific input validation checks (e.g., reserved bits,
    // valid function ID ranges) that would trigger an error other than NOT_SUPPORTED.
    // Therefore, the only defined failure mode is the explicit NOT_SUPPORTED return.
    (result == FFA_NOT_SUPPORTED)
    // Success condition: if the interface/feature is implemented, the callee returns FFA_SUCCESS.
    // The spec text does not define any preconditions that must hold for success other than
    // the implicit existence of the feature. The return value `result` encodes the properties
    // in w2-w3, but the spec text does not impose constraints on these values (e.g.,
    // specific bit patterns must be set) for the success path to be valid.
    // Thus, any return value other than FFA_NOT_SUPPORTED is considered a valid success
    // encoding the queried properties.
    && (result != FFA_NOT_SUPPORTED)
}