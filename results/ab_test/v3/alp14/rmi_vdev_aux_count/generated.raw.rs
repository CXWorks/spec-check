pub open spec fn rmi_vdev_aux_count_spec(result: RmiCommandReturnCode, aux_count: u64, pdev_flags: u64, vdev_flags: u64, old_s: S, new_s: S) -> bool {
    let pdev_flags_decoded = RmiPdevFlagsDecode(old_s, pdev_flags);
    let vdev_flags_decoded = RmiVdevFlagsDecode(old_s, vdev_flags);
    let expected_aux_count = VdevAuxCount(old_s, pdev_flags_decoded, vdev_flags_decoded);
    
    (!(old_s.ImplFeatures().feat_da == FEATURE_TRUE) ==> ResultEqual(Result::<(), RmiStatusCode>::Err(result), RMI_ERROR_NOT_SUPPORTED))
    && ((old_s.ImplFeatures().feat_da == FEATURE_TRUE) ==> (result == RMI_OK && aux_count == ToBits64(expected_aux_count) && new_s == old_s))
}