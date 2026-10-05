pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: SbiRet, old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, vmid: UInt64) -> bool {
    (!SbiRfenceExtensionImplemented(old_s) ==> result.error == SBI_ERR_NOT_SUPPORTED)
    && (SbiRfenceExtensionImplemented(old_s) && !TargetHartsImplementHypervisorExtension(old_s, hart_mask, hart_mask_base) ==> result.error == SBI_ERR_NOT_SUPPORTED)
    && (SbiRfenceExtensionImplemented(old_s) && TargetHartsImplementHypervisorExtension(old_s, hart_mask, hart_mask_base) && !HartMaskIsValid(old_s, hart_mask, hart_mask_base) ==> result.error == SBI_ERR_INVALID_PARAM)
    && (SbiRfenceExtensionImplemented(old_s) && TargetHartsImplementHypervisorExtension(old_s, hart_mask, hart_mask_base) && HartMaskIsValid(old_s, hart_mask, hart_mask_base) && !GuestPhysAddrRangeIsValid(old_s, start_addr, size) ==> result.error == SBI_ERR_INVALID_ADDRESS)
    && (result.error != SBI_SUCCESS ==> new_s == old_s)
    && ((SbiRfenceExtensionImplemented(old_s)
        && TargetHartsImplementHypervisorExtension(old_s, hart_mask, hart_mask_base)
        && HartMaskIsValid(old_s, hart_mask, hart_mask_base)
        && GuestPhysAddrRangeIsValid(old_s, start_addr, size))
        ==> (result.error == SBI_SUCCESS
            && RemoteHfenceGvmaVmidExecuted(old_s, new_s, hart_mask, hart_mask_base, start_addr, size, vmid)))
}
