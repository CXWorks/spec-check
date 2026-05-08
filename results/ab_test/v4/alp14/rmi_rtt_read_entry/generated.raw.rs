pub open spec fn RMI_RTT_READ_ENTRY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: Result<(), RmiStatusCode>,
    walk_level: u64,
    state: u8,
    desc: u64,
    ripas: u8
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, 0);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);
    
    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     (GranuleAt(old_s, rd).state != 2 ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     (!RttLevelIsValid(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     ((ipa as int) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
     (result.is_Ok() ==> (
       walk_level == (walk.level as u64) &&
       state == (RttEntryStateToRmi(old_s, walk.rtte.state) as u8) &&
       ((walk.rtte.state == 0 || walk.rtte.state == 1) ==> (
         rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == 0xFFFFFFFF &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == false &&
         rtte.s2ap_direct.write == false &&
         rtte.addr == 0
       )) &&
       ((walk.rtte.state == 2 || walk.rtte.state == 3 || walk.rtte.state == 4 || walk.rtte.state == 5) ==> (
         rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == 0xFFFFFFFF &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == false &&
         rtte.s2ap_direct.write == false &&
         rtte.addr == walk.rtte.addr
       )) &&
       (walk.rtte.state == 6 ==> (
         rtte.attr_unprot == walk.rtte.attr_unprot &&
         rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == walk.rtte.s2ap_direct.read &&
         rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write &&
         rtte.addr == walk.rtte.addr
       )) &&
       (walk.rtte.state == 3 ==> (
         rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == 0xFFFFFFFF &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == false &&
         rtte.s2ap_direct.write == false &&
         rtte.addr == walk.rtte.addr
       )) &&
       (walk.rtte.state == 4 ==> (
         rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == 0xFFFFFFFF &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == false &&
         rtte.s2ap_direct.write == false &&
         rtte.addr == walk.rtte.addr
       )) &&
       ((walk.rtte.state == 0 || walk.rtte.state == 2) ==> (ripas == (RipasToRmi(old_s, walk.rtte.ripas) as u8))) &&
       ((walk.rtte.state == 1 || walk.rtte.state == 6) ==> (ripas == 0))
     )))
}