```verus
pub open spec fn RMI_VDEV_CREATE_spec(s: S, rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address) -> bool {
  let realm_pre = RealmAt(s, rd);
  let pdev = PdevAt(s, pdev_ptr);
  let num_vdevs_pre = pdev.num_vdevs;
  let vdev = VdevAt(s, vdev_ptr);
  let params = RmiVdevParamsAt(s, params_ptr);
  let num_aux = VdevAuxCount(s, pdev.flags, params.flags);

  // Failure conditions
  let da_supp_fail = !ImplFeatures(s).feat_da;
  let rd_align_fail = !AddrIsGranuleAligned(s, rd);
  let rd_bound_fail = !PaIsDelegable(s, rd);
  let rd_state_fail = GranuleAt(s, rd).state != RD;
  let pdev_align_fail = !AddrIsGranuleAligned(s, pdev_ptr);
  let pdev_bound_fail = !PaIsDelegable(s, pdev_ptr);
  let pdev_gran_state_fail = GranuleAt(s, pdev_ptr).state != PDEV;
  let pdev_state_fail = pdev.state != PDEV_READY;
  let vdev_align_fail = !AddrIsGranuleAligned(s, vdev_ptr);
  let vdev_bound_fail = !PaIsDelegableDram(s, vdev_ptr);
  let vdev_gran_state_fail = GranuleAt(s, vdev_ptr).state != DELEGATED;
  let params_align_fail = !AddrIsGranuleAligned(s, params_ptr);
  let params_pas_fail = !GranuleAccessPermitted(s, params_ptr, PAS_NS);
  let params_valid_fail = !RmiVdevParamsIsValid(s, params_ptr);
  let da_en_fail = realm_pre.feat_da != FEATURE_TRUE;
  let num_aux_fail = params.num_aux != num_aux;
  let aux_align_fail = !AuxAligned32(s, params.aux, params.num_aux);
  let aux_alias_fail = AuxAlias32(s, vdev_ptr, params.aux, params.num_aux);
  let aux_state_fail = !AuxStateEqual32(s, params.aux, params.num_aux, DELEGATED);
  let vdev_id_free_fail = !VdevIdIsFree(s, realm_pre, params.vdev_id);
  let tdi_id_free_fail = !TdiIdIsFree(s, params.tdi_id, pdev.segment_id);
  let tdi_id_bound_fail = (params.tdi_id as int) < (pdev.rid_base as int) || (params.tdi_id as int) >= (pdev.rid_top as int);
  let vsmmu_align_fail = params.flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(s, params.vsmmu_addr);
  let vsmmu_bound_fail = params.flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(s, params.vsmmu_addr);
  let vsmmu_state_fail = params.flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(s, params.vsmmu_addr).state != VSMMU;
  let vsid_free_fail = params.flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(s, VsmmuAt(s, params.vsmmu_addr), params.vsid);
  let vsmmu_compat_fail = params.flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(s, pdev, VsmmuAt(s, params.vsmmu_addr));

  // Success conditions
  let pdev_num_vdevs_success = pdev.num_vdevs == num_vdevs_pre + 1;
  let gran_state_success = GranuleAt(s, vdev_ptr).state == VDEV;
  let vdev_id_success = vdev.vdev_id == params.vdev_id;
  let tdi_id_success = vdev.tdi_id == params.tdi_id;
  let pdev_success = vdev.pdev == pdev_ptr;
  let realm_success = vdev.realm == rd;
  let vdev_state_success = vdev.vdev_state == VDEV_NEW;
  let dma_state_success = vdev.dma_state == VDEV_DMA_DISABLED;
  let op_success = vdev.op == VDEV_OP_UNLOCK;
  let comm_state_success = vdev.comm_state == DEV_COMM_PENDING;
  let aux_success = AuxEqual32(s, vdev.aux, params.aux, num_aux);
  let num_aux_success = vdev.num_aux == num_aux;
  let aux_state_success = AuxStateEqual32(s, vdev.aux, num_aux, VDEV_AUX);
  let tdi_id_used_success = !TdiIdIsFree(s, params.tdi_id, pdev.segment_id);
  let vsmmu_success = vdev.vsmmu == params.flags.VSMMU;
  let vsmmu_addr_success = params.flags.VSMMU == RMI_FEATURE_TRUE ==> vdev.vsmmu_addr == params.vsmmu_addr;
  let vsid_success = params.flags.VSMMU == RMI_FEATURE_TRUE ==> vdev.vsid == params.vsid;
  let vsid_alloc_success = params.flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(s, VsmmuAt(s, params.vsmmu_addr), params.vsid);
  let num_map_success = vdev.num_map == 0;
  let realm_num_vdevs_success = realm_pre.num_vdevs + 1 == realm_pre.num_vdevs + 1;
  let lock_nonce_success = vdev.attest_info.lock_nonce == 0;
  let meas_nonce_success = vdev.attest_info.meas_nonce == 0;
  let report_nonce_success = vdev.attest_info.report_nonce == 0;
  let p2p_bound_success = vdev.p2p_bound == FEATURE_FALSE;

  // Either all success conditions hold, or one of the failure conditions holds
  (pdev_num_vdevs_success && gran_state_success && vdev_id_success && tdi_id_success && pdev_success && realm_success && vdev_state_success && dma_state_success && op_success && comm_state_success && aux_success && num_aux_success && aux_state_success && tdi_id_used_success && vsmmu_success && vsmmu_addr_success && vsid_success && vsid_alloc_success && num_map_success && lock_nonce_success && meas_nonce_success && report_nonce_success && p2p_bound_success) || 
  (da_supp_fail || rd_align_fail || rd_bound_fail || rd_state_fail || pdev_align_fail || pdev_bound_fail || pdev_gran_state_fail || pdev_state_fail || vdev_align_fail || vdev_bound_fail || vdev_gran_state_fail || params_align_fail || params_pas_fail || params_valid_fail || da_en_fail || num_aux_fail || aux_align_fail || aux_alias_fail || aux_state_fail || v