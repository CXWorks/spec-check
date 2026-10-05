pub open spec fn ffa_id_get_spec(result: FfaReturnCode, id: UInt32, old_s: S, new_s: S) -> bool {
    (!FfaIdGetImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && (FfaIdGetImplemented(old_s) ==> (
        result == FFA_SUCCESS
        && (id >> 16u32) == 0u32
        && ((id & 0xFFFFu32) as int) == (CallerFfaId(old_s) as int)
        && (IsNonSecurePhysicalFfaInstance(old_s) ==> id == 0u32)
        && new_s == old_s
    ))
}
