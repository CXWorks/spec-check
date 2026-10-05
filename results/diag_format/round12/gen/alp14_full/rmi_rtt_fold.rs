pub open spec fn rmi_rtt_fold_spec(rd: Rd, ipa: Address, level: UInt64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err() ==> RealmAt(new_s, rd).rtt[ipa as int] == RealmAt(old_s, rd).rtt[ipa as int])
  && ((!(GranuleAt(old_s, rd).state == RD) &&
       result.is_Ok())
    ==> GranuleAt(new_s, rd).state == DELEGATED)
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt[ipa as int].state == RealmAt(old_s, rd).rtt[ipa as int].state)
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt[ipa as int].output_addr == RealmAt(old_s, rd).rtt[ipa as int].output_addr)
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt[ipa as int].ripas == RealmAt(old_s, rd).rtt[ipa as int].ripas)
  && ((!(GranuleAt(old_s, rd).state == RD) &&
       result.is_Err())
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt[ipa as int] == RealmAt(old_s, rd).rtt[ipa as int])
}