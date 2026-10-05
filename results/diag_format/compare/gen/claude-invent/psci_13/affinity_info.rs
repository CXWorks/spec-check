pub open spec fn affinity_info_spec(target_affinity: UInt64, lowest_affinity_level: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level as int) ==> result == INVALID_PARAMETERS)
    && ((AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level as int)
         && AffinityInstanceIsDisabled(old_s, target_affinity, lowest_affinity_level as int)) ==> result == DISABLED)
    && ((AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level as int)
         && !AffinityInstanceIsDisabled(old_s, target_affinity, lowest_affinity_level as int)
         && (lowest_affinity_level as int) == 0) ==> (
            (AffinityInstanceAnyCoreOn(old_s, target_affinity, lowest_affinity_level as int) ==> result == ON)
            && ((!AffinityInstanceAnyCoreOn(old_s, target_affinity, lowest_affinity_level as int)
                 && AffinityInstanceAnyCoreOnPending(old_s, target_affinity, lowest_affinity_level as int)) ==> result == ON_PENDING)
            && (AffinityInstanceAllCoresOff(old_s, target_affinity, lowest_affinity_level as int) ==> result == OFF)
        ))
    && ((AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level as int)
         && !AffinityInstanceIsDisabled(old_s, target_affinity, lowest_affinity_level as int)
         && (lowest_affinity_level as int) > 0) ==> (
            result == INVALID_PARAMETERS
            || (
                (AffinityInstanceAnyCoreOn(old_s, target_affinity, lowest_affinity_level as int) ==> result == ON)
                && ((!AffinityInstanceAnyCoreOn(old_s, target_affinity, lowest_affinity_level as int)
                     && AffinityInstanceAnyCoreOnPending(old_s, target_affinity, lowest_affinity_level as int)) ==> result == ON_PENDING)
                && (AffinityInstanceAllCoresOff(old_s, target_affinity, lowest_affinity_level as int) ==> result == OFF)
            )
        ))
    && new_s == old_s
}
