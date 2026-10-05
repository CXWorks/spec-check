pub open spec fn hci_volume_validate_mapping_spec(vd: PhysicalAddress, worker_ptr: PhysicalAddress, drive_ptr: PhysicalAddress, volume_ptr: PhysicalAddress, base: UInt64, top: UInt64, result: Result<HciStatusCode, (PhysicalAddress,)>, out_top: UInt64, old_s: S, new_s: S) -> bool {
  (vd % ExtentSize(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!IsEnrollablePhysicalAddress(old_s, vd) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, vd).state != VD_STATE ==> HCI_ERROR_INPUT(result))
  && (worker_ptr % ExtentSize(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!IsEnrollablePhysicalAddress(old_s, worker_ptr) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, worker_ptr).state != WORKER_STATE ==> HCI_ERROR_INPUT(result))
  && (WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING ==> HCI_ERROR_WORKER(result))
  && (!WorkerOwnedByVault(old_s, worker_ptr, vd) ==> HCI_ERROR_WORKER(result))
  && (drive_ptr % ExtentSize(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!IsEnrollablePhysicalAddress(old_s, drive_ptr) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, drive_ptr).state != DRIVE_STATE ==> HCI_ERROR_INPUT(result))
  && (volume_ptr % ExtentSize(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!IsEnrollablePhysicalAddress(old_s, volume_ptr) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, volume_ptr).state != VOLUME_STATE ==> HCI_ERROR_INPUT(result))
  && (VolumeAssociatedWithDrive(old_s, volume_ptr, drive_ptr) ==> HCI_ERROR_DEVICE(result))
  && (top <= base ==> HCI_ERROR_INPUT(result))
  && (WorkerAt(old_s, worker_ptr).dev_mem_addr != base ==> HCI_ERROR_INPUT(result))
  && (top > WorkerAt(old_s, worker_ptr).dev_mem_top ==> HCI_ERROR_INPUT(result))
  && (base % BltEntrySize(old_s) != 0 ==> HCI_ERROR_BLT(result))
  && (top % ExtentSize(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (base == TopOfWalkedRange(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (WorkerAt(old_s, worker_ptr).dev_mem_non_coherent && !every_blt_entry_in_walked_range_has_memattr_non_cacheable(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (WorkerAt(old_s, worker_ptr).dev_mem_non_coherent && !every_blt_entry_in_walked_range_maps_non_coherent_device_memory(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (WorkerAt(old_s, worker_ptr).dev_mem_coherent && !every_blt_entry_in_walked_range_has_memattr_passthrough(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (WorkerAt(old_s, worker_ptr).dev_mem_coherent && !every_blt_entry_in_walked_range_maps_coherent_device_memory(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (BltEntriesInWalkedRangeHaveNonContiguousOutputAddresses(old_s, worker_ptr, base) ==> HCI_ERROR_BLT(result))
  && (AnyAddressInRangeFromBaseToTopIsLiveInAuxiliaryBlt(old_s, worker_ptr, base, top) ==> HCI_ERROR_BLT(result))
  && (result == HCI_SUCCESS ==> LBAMODEOfEveryBltEntryInWalkedRange(new_s, worker_ptr, base) == DEV)
  && (result == HCI_SUCCESS ==> WorkerAt(new_s, worker_ptr).dev_mem_addr == min(top, TopOfWalkedRange(new_s, worker_ptr, base)))
  && (result == HCI_SUCCESS ==> WorkerAt(new_s, worker_ptr).dev_mem_dpa == WorkerAt(new_s, worker_ptr).dev_mem_dpa + (top - base))
  && (result == HCI_SUCCESS ==> out_top == min(top, TopOfWalkedRange(new_s, worker_ptr, base)))
  && ((!(vd % ExtentSize(old_s) != 0) &&
       IsEnrollablePhysicalAddress(old_s, vd) &&
       !(ExtentAt(old_s, vd).state != VD_STATE) &&
       !(worker_ptr % ExtentSize(old_s) != 0) &&
       IsEnrollablePhysicalAddress(old_s, worker_ptr) &&
       !(ExtentAt(old_s, worker_ptr).state != WORKER_STATE) &&
       !(WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING) &&
       WorkerOwnedByVault(old_s, worker_ptr, vd) &&
       !(drive_ptr % ExtentSize(old_s) != 0) &&
       IsEnrollablePhysicalAddress(old_s, drive_ptr) &&
       !(ExtentAt(old_s, drive_ptr).state != DRIVE_STATE) &&
       !(volume_ptr % ExtentSize(old_s) != 0) &&
       IsEnrollablePhysicalAddress(old_s, volume_ptr) &&
       !(ExtentAt(old_s, volume_ptr).state != VOLUME_STATE) &&
       VolumeAssociatedWithDrive(old_s, volume_ptr, drive_ptr) &&
       !(top <= base) &&
       !(WorkerAt(old_s, worker_ptr).dev_mem_addr != base) &&
       !(top > WorkerAt(old_s, worker_ptr).dev_mem_top) &&
       !(base % BltEntrySize(old_s) != 0) &&
       !(top % ExtentSize(old_s) != 0) &&
       !(base == TopOfWalkedRange(old_s, worker_ptr, base)) &&
       !((WorkerAt(old_s, worker_ptr).dev_mem_non_coherent) && !every_blt_entry_in_walked_range_has_memattr_non_cacheable(old_s, worker_ptr, base)) &&
       !((WorkerAt(old_s, worker_ptr).dev_mem_non_coherent) && !every_blt_entry_in_walked_range_maps_non_coherent_device_memory(old_s, worker_ptr, base)) &&
       !((WorkerAt(old_s, worker_ptr).dev_mem_coherent) && !every_blt_entry_in_walked_range_has_memattr_passthrough(old_s, worker_ptr, base)) &&
       !((WorkerAt(old_s, worker_ptr).dev_mem_coherent) && !every_blt_entry_in_walked_range_maps_coherent_device_memory(old_s, worker_ptr, base)) &&
       !(BltEntriesInWalkedRangeHaveNonContiguousOutputAddresses(old_s, worker_ptr, base)) &&
       !(AnyAddressInRangeFromBaseToTopIsLiveInAuxiliaryBlt(old_s, worker_ptr, base, top)))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> WorkerAt(new_s, worker_ptr).dev_mem_addr == WorkerAt(old_s, worker_ptr).dev_mem_addr)
  && (result != HCI_SUCCESS
    ==> WorkerAt(new_s, worker_ptr).dev_mem_dpa == WorkerAt(old_s, worker_ptr).dev_mem_dpa)
  && (!(result == HCI_SUCCESS) ==> WorkerAt(new_s, worker_ptr).dev_mem_addr == WorkerAt(old_s, worker_ptr).dev_mem_addr)
}