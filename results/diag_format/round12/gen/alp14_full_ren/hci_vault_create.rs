pub open spec fn hci_vault_create_spec(vd: UInt64, params_ptr: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result != RSI_SUCCESS ==> ExtentAt(new_s, vd).state == ENROLLED)
  && (result == RSI_SUCCESS ==> ExtentAt(new_s, vd).state == VD)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).state == VAULT_NEW)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).work_index == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).blt_base == VaultParamsAt(new_s, params_ptr).blt_base)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).aux_blt_base == VaultParamsAt(new_s, params_ptr).aux_blt_base)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).ifp == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).rem[0] == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).rem[1] == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).rem[2] == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).rem[3] == 0)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).blt_level_start == VaultParamsAt(new_s, params_ptr).blt_level_start)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).blt_num_start == VaultParamsAt(new_s, params_ptr).blt_num_start)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).tenantid == VaultParamsAt(new_s, params_ptr).tenantid)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).aux_tenantid == VaultParamsAt(new_s, params_ptr).aux_tenantid)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).ptv == VaultParamsAt(new_s, params_ptr).ptv)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).xlc_tier == VaultParamsAt(new_s, params_ptr).xlc_tier)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, vd).zoneid == VaultParamsAt(new_s, params_ptr).zoneid)
  && ((!(params_ptr % extent_size as int == 0) ||
       ExtentAt(old_s, params_ptr).state != ACCESSIBLE)
    ==> result != RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       VaultParamsAt(old_s, params_ptr).blt_base + (VaultParamsAt(old_s, params_ptr).blt_num_start - 1) * extent_size <= vd
    ==> result != RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       !(ExtentAt(old_s, vd).state == ENROLLED)
    ==> result != RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       VaultParamsAt(old_s, params_ptr).blt_base % (VaultParamsAt(old_s, params_ptr).blt_num_start as int) != 0
    ==> result != RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       VaultParamsAt(old_s, params_ptr).zoneid > 0
    ==> result != RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       ZoneStateAt(old_s, VaultParamsAt(old_s, params_ptr).zoneid) == ZONE_STATE_PRIVATE_ASSIGNED
    ==> result != RSI_SUCCESS)
  && ((!(params_ptr % extent_size as int == 0) &&
       ExtentAt(old_s, params_ptr).state == ACCESSIBLE)
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       !(VaultParamsAt(old_s, params_ptr).blt_base + (VaultParamsAt(old_s, params_ptr).blt_num_start - 1) * extent_size <= vd))
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       ExtentAt(old_s, vd).state == ENROLLED)
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       VaultParamsAt(old_s, params_ptr).blt_base % (VaultParamsAt(old_s, params_ptr).blt_num_start as int) == 0)
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       !(VaultParamsAt(old_s, params_ptr).zoneid > 0))
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS &&
       !(ZoneStateAt(old_s, VaultParamsAt(old_s, params_ptr).zoneid) == ZONE_STATE_PRIVATE_ASSIGNED))
    ==> result == RSI_SUCCESS)
  && ((result == RSI_SUCCESS)
    ==> ZoneStateAt(new_s, VaultParamsAt(new_s, params_ptr).zoneid) == ZONE_STATE_PRIVATE_ASSIGNED)
  && (result == RSI_SUCCESS &&
       ZoneStateAt(old_s, VaultParamsAt(old_s, params_ptr).zoneid) == ZONE_STATE_SHARED)
    ==> ZoneMemberCount(new_s, VaultParamsAt(new_s, params_ptr).zoneid) == ZoneMemberCount(old_s, VaultParamsAt(old_s, params_ptr).zoneid) + 1)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt(new_s, vd).state == ExtentAt(old_s, vd).state)
  && ((!(result == RSI_SUCCESS) ||
       !(result == RSI_SUCCESS))
    ==> ExtentAt