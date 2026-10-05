pub open spec fn protocol_attributes__3_12_4_3_spec(status: Int32, de_num: UInt32, groups_num: UInt32, de_implementation_rev_dword0: UInt32, de_implementation_rev_dword1: UInt32, de_implementation_rev_dword2: UInt32, de_implementation_rev_dword3: UInt32, attributes_1: UInt32, default_blk_ts_rate: UInt32, old_s: S, new_s: S) -> bool {
  (ResultEqual(status, SUCCESS))
  && (de_num == NumDataEventsSupported())
  && (groups_num == NumEventGroupsSupported())
  && (DeImplementationRev128(de_implementation_rev_dword0, de_implementation_rev_dword1, de_implementation_rev_dword2, de_implementation_rev_dword3) == PrimaryDeImplementationRevision())
  && (Bits(attributes_1, 31, 31) == 1 ==> SupportsSingleSampleAsyncReadViaConfigSet() && ReadCompletionSignalledByTelemetryReadingComplete())
  && (Bits(attributes_1, 31, 31) == 0 ==> !SupportsSingleSampleAsyncReadViaConfigSet())
  && (Bits(attributes_1, 30, 30) == 1 ==> SupportsContinuousUpdateNotifications() && TelemetryUpdateSentAtSamplingInterval())
  && (Bits(attributes_1, 30, 30) == 0 ==> !SupportsContinuousUpdateNotifications())
  && (Bits(attributes_1, 29, 19) == 0)
  && (Bits(attributes_1, 18, 18) == 1 ==> PerEventGroupSamplingRateAndCollectionModeConfigurable() && UngroupedDesShareSamplingRateAndCollectionMode())
  && (Bits(attributes_1, 18, 18) == 0 ==> AllDesAndGroupsShareSamplingRateAndCollectionMode())
  && (Bits(attributes_1, 17, 17) == 1 ==> SupportsFullTelemetryReset())
  && (Bits(attributes_1, 16, 16) == 1 ==> FastChannelsSupported() && (exists|de: DataEvent| UsesFastChannel(de)))
  && (Bits(attributes_1, 16, 16) == 0 ==> !FastChannelsSupported())
  && (Bits(attributes_1, 15, 0) == NumShmtisSupported())
  && (default_blk_ts_rate == DefaultBlockTimestampClockRateKhz())
  && (forall|ts: BlockTimestampLine| ts.id == 0 ==> BlockTimestampClockRateKhz(ts) == default_blk_ts_rate)
  && ((!(ResultEqual(status, SUCCESS)) &&
       !(de_num == NumDataEventsSupported()) &&
       !(groups_num == NumEventGroupsSupported()) &&
       !(DeImplementationRev128(de_implementation_rev_dword0, de_implementation_rev_dword1, de_implementation_rev_dword2, de_implementation_rev_dword3) == PrimaryDeImplementationRevision()) &&
       !(Bits(attributes_1, 31, 31) == 1 ==> SupportsSingleSampleAsyncReadViaConfigSet() && ReadCompletionSignalledByTelemetryReadingComplete()) &&
       !(Bits(attributes_1, 31, 31) == 0 ==> !SupportsSingleSampleAsyncReadViaConfigSet()) &&
       !(Bits(attributes_1, 30, 30) == 1 ==> SupportsContinuousUpdateNotifications() && TelemetryUpdateSentAtSamplingInterval()) &&
       !(Bits(attributes_1, 30, 30) == 0 ==> !SupportsContinuousUpdateNotifications()) &&
       !(Bits(attributes_1, 29, 19) == 0) &&
       !(Bits(attributes_1, 18, 18) == 1 ==> PerEventGroupSamplingRateAndCollectionModeConfigurable() && UngroupedDesShareSamplingRateAndCollectionMode()) &&
       !(Bits(attributes_1, 18, 18) == 0 ==> AllDesAndGroupsShareSamplingRateAndCollectionMode()) &&
       !(Bits(attributes_1, 17, 17) == 1 ==> SupportsFullTelemetryReset()) &&
       !(Bits(attributes_1, 16, 16) == 1 ==> FastChannelsSupported() && (exists|de: DataEvent| UsesFastChannel(de))) &&
       !(Bits(attributes_1, 16, 16) == 0 ==> !FastChannelsSupported()) &&
       !(Bits(attributes_1, 15, 0) == NumShmtisSupported()) &&
       !(default_blk_ts_rate == DefaultBlockTimestampClockRateKhz()) &&
       !(forall|ts: BlockTimestampLine| ts.id == 0 ==> BlockTimestampClockRateKhz(ts) == default_blk_ts_rate))
    ==> (status == 0 &&
         de_num == 0 &&
         groups_num == 0 &&
         de_implementation_rev_dword0 == 0 &&
         de_implementation_rev_dword1 == 0 &&
         de_implementation_rev_dword2 == 0 &&
         de_implementation_rev_dword3 == 0 &&
         attributes_1 == 0 &&
         default_blk_ts_rate == 0))
}