pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (
        (VersionEqualRsi(lower, req) ==> VersionEqualRsi(higher, req))
        || (lower.major < req.major || (lower.major == req.major && lower.minor < req.minor))
    ))
    && (result == RSI_SUCCESS ==> (
        VersionEqualRsi(lower, req) && VersionEqualRsi(higher, RsiVersionHighest(old_s))
    ))
    && (result != RSI_SUCCESS ==> (
        (lower.major < req.major || (lower.major == req.major && lower.minor < req.minor))
    ))
    && (result == RSI_SUCCESS ==> (
        VersionEqualRsi(lower, req) && VersionEqualRsi(higher, RsiVersionHighest(old_s))
    ))
}