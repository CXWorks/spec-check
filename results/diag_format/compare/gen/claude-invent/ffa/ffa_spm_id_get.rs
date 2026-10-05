pub open spec fn ffa_spm_id_get_spec(ret_fid: UInt32, w2: UInt32, old_s: S, new_s: S) -> bool {
    (!FfaSpmIdGetImplementedAtInstance(old_s) ==> (ret_fid == FFA_ERROR && w2 == NOT_SUPPORTED && new_s == old_s))
    && (FfaSpmIdGetImplementedAtInstance(old_s) ==> (
        ret_fid == FFA_SUCCESS
        && (w2 >> 16u32) == 0
        && new_s == old_s
        && ((FfaInstanceIsNonSecurePhysical(old_s) || FfaInstanceIsNonSecureVirtual(old_s)) ==> ((w2 & 0xFFFFu32) as int == SpmcId(old_s) as int))
        && (FfaInstanceIsSecureVirtual(old_s) ==> ((w2 & 0xFFFFu32) as int == SpmcId(old_s) as int))
        && ((FfaInstanceIsSecurePhysical(old_s) && !SpmcImplementedAtEl3(old_s)) ==> ((w2 & 0xFFFFu32) as int == SpmdId(old_s) as int))
        && ((FfaInstanceIsSecurePhysical(old_s) && SpmcImplementedAtEl3(old_s)) ==> ((w2 & 0xFFFFu32) as int == SpmcId(old_s) as int))
    ))
}
