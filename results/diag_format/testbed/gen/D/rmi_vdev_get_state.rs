pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
    (!crate::RmmFeature::feat_da_supported(old_s) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!crate::AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (crate::GranuleAt(old_s, vdev_ptr).state != crate::RmmGranuleState::VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> state == crate::GranuleAt(old_s, vdev_ptr).state)
}