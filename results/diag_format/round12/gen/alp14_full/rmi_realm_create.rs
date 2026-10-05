pub open spec fn rmi_realm_create_spec(rd: UInt, params_ptr: UInt, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (GranuleAt(new_s, rd).state != RD ==> result.is_Err())
  && (RealmAt(new_s, rd).state != REALM_NEW ==> result.is_Err())
  && (RealmAt(new_s, rd).rec_index != 0 ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_base != RealmAt(new_s, rd).rtt_base ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_aux_base != RealmAt(new_s, rd).rtt_aux_base ==> result.is_Err())
  && (RealmAt(new_s, rd).lpa2 != RealmAt(new_s, rd).lpa2 ==> result.is_Err())
  && (RealmAt(new_s, rd).ipa_width != RealmAt(new_s, rd).ipa_width ==> result.is_Err())
  && (RealmAt(new_s, rd).hash_algo != RealmAt(new_s, rd).hash_algo ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_level_start != RealmAt(new_s, rd).rtt_level_start ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_num_start != RealmAt(new_s, rd).rtt_num_start ==> result.is_Err())
  && (RealmAt(new_s, rd).vmid != RealmAt(new_s, rd).vmid ==> result.is_Err())
  && (RealmAt(new_s, rd).aux_vmid != RealmAt(new_s, rd).aux_vmid ==> result.is_Err())
  && (RealmAt(new_s, rd).rpv != RealmAt(new_s, rd).rpv ==> result.is_Err())
  && (RealmAt(new_s, rd).da != RealmAt(new_s, rd).da ==> result.is_Err())
  && (RealmAt(new_s, rd).ats != RealmAt(new_s, rd).ats ==> result.is_Err())
  && (RealmAt(new_s, rd).ats_plane != RealmAt(new_s, rd).ats_plane ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_tree_per_plane != RealmAt(new_s, rd).rtt_tree_per_plane ==> result.is_Err())
  && (RealmAt(new_s, rd).num_aux_planes != RealmAt(new_s, rd).num_aux_planes ==> result.is_Err())
  && (RealmAt(new_s, rd).rtt_s2ap_encoding != RealmAt(new_s, rd).rtt_s2ap_encoding ==> result.is_Err())
  && (RealmAt(new_s, rd).lfa_policy != RealmAt(new_s, rd).lfa_policy ==> result.is_Err())
  && (RealmAt(new_s, rd).mecid != RealmAt(new_s, rd).mecid ==> result.is_Err())
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == RD)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rec_index == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_base == RealmAt(new_s, rd).rtt_base)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_aux_base == RealmAt(new_s, rd).rtt_aux_base)
  && (result.is_Ok() ==> RealmAt(new_s, rd).lpa2 == RealmAt(new_s, rd).lpa2)
  && (result.is_Ok() ==> RealmAt(new_s, rd).ipa_width == RealmAt(new_s, rd).ipa_width)
  && (result.is_Ok() ==> RealmAt(new_s, rd).hash_algo == RealmAt(new_s, rd).hash_algo)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(new_s, rd).rtt_level_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(new_s, rd).rtt_num_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).vmid == RealmAt(new_s, rd).vmid)
  && (result.is_Ok() ==> RealmAt(new_s, rd).aux_vmid == RealmAt(new_s, rd).aux_vmid)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rpv == RealmAt(new_s, rd).rpv)
  && (result.is_Ok() ==> RealmAt(new_s, rd).da == RealmAt(new_s, rd).da)
  && (result.is_Ok() ==> RealmAt(new_s, rd).ats == RealmAt(new_s, rd).ats)
  && (result.is_Ok() ==> RealmAt(new_s, rd).ats_plane == RealmAt(new_s, rd).ats_plane)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_tree_per_plane == RealmAt(new_s, rd).rtt_tree_per_plane)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_aux_planes == RealmAt(new_s, rd).num_aux_planes)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_s2ap_encoding == RealmAt(new_s, rd).rtt_s2ap_encoding)
  && (result.is_Ok() ==> RealmAt(new_s, rd).lfa_policy == RealmAt(new_s, rd).lfa_policy)
  && (result.is_Ok() ==> RealmAt(new_s, rd).mecid == RealmAt(new_s, rd).mecid)
  && ((!(GranuleAt(old_s, rd).state != RD) &&
       RealmAt(old_s, rd).state == REALM_NEW &&
       RealmAt(old_s, rd).rec_index == 0 &&
       RealmAt(old_s, rd).rtt_base == RealmAt(old_s, rd).rtt_base &&
       RealmAt(old_s, rd).rtt_aux_base == RealmAt(old_s, rd).rtt_aux_base &&
       RealmAt(old_s, rd).lpa2 == RealmAt(old_s, rd).lpa2 &&
       RealmAt(old_s, rd).ipa_width == RealmAt(old_s, rd).ipa_width &&
       RealmAt(old_s, rd).hash_algo == RealmAt(old_s, rd).hash_algo &&
       RealmAt(old_s, rd).rtt_level_start == RealmAt(old_s, rd).rtt_level_start &&
       RealmAt(old_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start &&
       RealmAt(old_s, rd).vmid == RealmAt(old_s, rd).vmid &&
       RealmAt(old_s, rd).aux_vmid == RealmAt(old_s, rd).aux_vmid &&
       RealmAt(old_s, rd).rpv == RealmAt(old_s, rd).rpv &&
       RealmAt(old_s, rd).da == RealmAt(old_s, rd).da &&
       RealmAt(old_s, rd).ats == RealmAt(old_s, rd).ats &&
       RealmAt(old_s, rd).ats_plane == RealmAt(old_s, rd).ats_plane &&
       RealmAt(old_s, rd).rtt_tree_per_plane == RealmAt(old_s, rd).rtt_tree_per_plane &&
       RealmAt(old_s, rd).num_aux_planes == RealmAt(old_s, rd).num_aux_planes &&
       RealmAt(old_s, rd).rtt_s2ap_encoding == RealmAt(old_s, rd).rtt_s2ap_encoding &&
       RealmAt(old_s, rd).lfa_policy == RealmAt(old_s, rd).lfa_policy &&
       RealmAt(old_s, rd).mecid == RealmAt(old_s, rd).mecid)
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_base == RealmAt(old_s, rd).rtt_base)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_aux_base == RealmAt(old_s, rd).rtt_aux_base)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).lpa2 == RealmAt(old_s, rd).lpa2)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ipa_width == RealmAt(old_s, rd).ipa_width)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).hash_algo == RealmAt(old_s, rd).hash_algo)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(old_s, rd).rtt_level_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).vmid == RealmAt(old_s, rd).vmid)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).aux_vmid == RealmAt(old_s, rd).aux_vmid)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rpv == RealmAt(old_s, rd).rpv)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).da == RealmAt(old_s, rd).da)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ats == RealmAt(old_s, rd).ats)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ats_plane == RealmAt(old_s, rd).ats_plane)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_tree_per_plane == RealmAt(old_s, rd).rtt_tree_per_plane)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_aux_planes == RealmAt(old_s, rd).num_aux_planes)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_s2ap_encoding == RealmAt(old_s, rd).rtt_s2ap_encoding)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).lfa_policy == RealmAt(old_s, rd).lfa_policy)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mecid == RealmAt(old_s, rd).mecid)
}