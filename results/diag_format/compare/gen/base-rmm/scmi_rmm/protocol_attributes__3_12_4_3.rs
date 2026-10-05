pub open spec fn protocol_attributes__3_12_4_3_spec(result: Int32, de_num: UInt32, groups_num: UInt32, de_implementation_rev_dword0: UInt32, de_implementation_rev_dword1: UInt32, de_implementation_rev_dword2: UInt32, de_implementation_rev_dword3: UInt32, attributes_1: UInt32, default_blk_ts_rate: UInt32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (de_num == NumDataEventsSupported())
    && (groups_num == NumEventGroupsSupported())
    && ((de_implementation_rev_dword3 as u64) << 96 | (de_implementation_rev_dword2 as u64) << 64 | (de_implementation_rev_dword1 as u64) << 32 | de_implementation_rev_dword0 as u64 == PrimaryDeImplementationRevision())
    && (attributes_1[31] == SupportsSingleSampleAsyncRead())
    && (attributes_1[30] == SupportsContinuousUpdateNotification())
    && (attributes_1[29:19] == 0)
    && (attributes_1[18] == SupportsEventGroupSpecificSampling())
    && (attributes_1[17] == SupportsTelemetryReset())
    && (attributes_1[16] == SupportsFastChannels())
    && (attributes_1[15:0] == NumShmtisSupported())
    && (default_blk_ts_rate == DefaultBlockTimestampClockRateKhz())
}