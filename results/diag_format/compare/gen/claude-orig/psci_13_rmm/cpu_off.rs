pub open spec fn cpu_off_spec(fid: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsTrustedOsResident(old_s, CallingCore(old_s)) ==> result == DENIED)
    && (!IsTrustedOsResident(old_s, CallingCore(old_s)) ==> (
        !ReturnsToCaller(new_s)
        && PowerState(new_s, CallingCore(old_s)) == POWERED_DOWN
        && CachesClean(new_s, CallingCore(old_s))
        && !IsCoherent(new_s, CallingCore(old_s))
    ))
}
