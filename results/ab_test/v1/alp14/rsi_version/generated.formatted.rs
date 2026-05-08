```verus
pub open spec fn RSI_VERSION_spec(s: S, req: RsiInterfaceVersion) -> (result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion) {
    let higher = RsiVersionHighest(s);
    if RsiVersionIsSupported(s, req) {
        // Success condition: lower
        (RSI_SUCCESS, req, higher)
    } else if RsiVersionLowerIsSupported(s, req) {
        // Failure condition: incompat_lower
        (RSI_ERROR_INPUT, RsiVersionHighestBelow(s, req), higher)
    } else if RsiVersionHigherIsSupported(s, req) {
        // Failure condition: incompat_higher
        (RSI_ERROR_INPUT, higher, higher)
    } else {
        // Should not occur given the specification constraints
        (RSI_ERROR_INPUT, higher, higher)
    }
}
```

This function captures the RSI_VERSION command's behavior:

1. **Success case**: If the requested version is supported, return `RSI_SUCCESS` with the requested version as `lower` and the highest supported version as `higher`.

2. **Failure case (incompat_lower)**: If the requested version is not supported but there exists a lower supported version, return `RSI_ERROR_INPUT` with the highest supported version below the requested one as `lower` and the highest supported version as `higher`.

3. **Failure case (incompat_higher)**: If the requested version is not supported and there's no lower supported version but there's a higher supported version, return `RSI_ERROR_INPUT` with both `lower` and `higher` set to the highest supported version.