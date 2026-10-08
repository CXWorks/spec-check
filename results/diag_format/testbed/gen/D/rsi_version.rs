pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
    (!VersionEqualRsi(req, higher) && req > lower ==> result == RSI_ERROR_INPUT)
    && (!VersionEqualRsi(req, higher) && req < higher ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> lower == req)
    && (result == RSI_SUCCESS ==> higher == RsiVersionHighest(old_s))
}