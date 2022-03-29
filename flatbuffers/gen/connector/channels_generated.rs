// automatically generated by the FlatBuffers compiler, do not modify



use crate::shared_generated::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum ChannelSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ChannelSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ChannelSettings<'a> {
    type Inner = ChannelSettings<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> ChannelSettings<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ChannelSettings { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ChannelSettingsArgs) -> flatbuffers::WIPOffset<ChannelSettings<'bldr>> {
      let mut builder = ChannelSettingsBuilder::new(_fbb);
      builder.add_min_htlc_msat(args.min_htlc_msat);
      builder.add_chan_reserve_sat(args.chan_reserve_sat);
      builder.finish()
    }

    pub const VT_CHAN_RESERVE_SAT: flatbuffers::VOffsetT = 4;
    pub const VT_MIN_HTLC_MSAT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn chan_reserve_sat(&self) -> u64 {
    self._tab.get::<u64>(ChannelSettings::VT_CHAN_RESERVE_SAT, Some(0)).unwrap()
  }
  #[inline]
  pub fn min_htlc_msat(&self) -> u64 {
    self._tab.get::<u64>(ChannelSettings::VT_MIN_HTLC_MSAT, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for ChannelSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>(&"chan_reserve_sat", Self::VT_CHAN_RESERVE_SAT, false)?
     .visit_field::<u64>(&"min_htlc_msat", Self::VT_MIN_HTLC_MSAT, false)?
     .finish();
    Ok(())
  }
}
pub struct ChannelSettingsArgs {
    pub chan_reserve_sat: u64,
    pub min_htlc_msat: u64,
}
impl<'a> Default for ChannelSettingsArgs {
    #[inline]
    fn default() -> Self {
        ChannelSettingsArgs {
            chan_reserve_sat: 0,
            min_htlc_msat: 0,
        }
    }
}
pub struct ChannelSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ChannelSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_chan_reserve_sat(&mut self, chan_reserve_sat: u64) {
    self.fbb_.push_slot::<u64>(ChannelSettings::VT_CHAN_RESERVE_SAT, chan_reserve_sat, 0);
  }
  #[inline]
  pub fn add_min_htlc_msat(&mut self, min_htlc_msat: u64) {
    self.fbb_.push_slot::<u64>(ChannelSettings::VT_MIN_HTLC_MSAT, min_htlc_msat, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ChannelSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ChannelSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ChannelSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ChannelSettings<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ChannelSettings");
      ds.field("chan_reserve_sat", &self.chan_reserve_sat());
      ds.field("min_htlc_msat", &self.min_htlc_msat());
      ds.finish()
  }
}
pub enum ChannelStateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ChannelState<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ChannelState<'a> {
    type Inner = ChannelState<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> ChannelState<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ChannelState { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ChannelStateArgs<'args>) -> flatbuffers::WIPOffset<ChannelState<'bldr>> {
      let mut builder = ChannelStateBuilder::new(_fbb);
      builder.add_unsettled_balance(args.unsettled_balance);
      builder.add_remote_balance(args.remote_balance);
      builder.add_local_balance(args.local_balance);
      builder.add_capacity(args.capacity);
      builder.add_short_channel_id(args.short_channel_id);
      if let Some(x) = args.remote_channel_settings { builder.add_remote_channel_settings(x); }
      if let Some(x) = args.local_channel_settings { builder.add_local_channel_settings(x); }
      if let Some(x) = args.remote_node_id { builder.add_remote_node_id(x); }
      if let Some(x) = args.local_node_id { builder.add_local_node_id(x); }
      builder.add_private(args.private);
      builder.add_active(args.active);
      builder.finish()
    }

    pub const VT_SHORT_CHANNEL_ID: flatbuffers::VOffsetT = 4;
    pub const VT_LOCAL_NODE_ID: flatbuffers::VOffsetT = 6;
    pub const VT_REMOTE_NODE_ID: flatbuffers::VOffsetT = 8;
    pub const VT_ACTIVE: flatbuffers::VOffsetT = 10;
    pub const VT_PRIVATE: flatbuffers::VOffsetT = 12;
    pub const VT_CAPACITY: flatbuffers::VOffsetT = 14;
    pub const VT_LOCAL_BALANCE: flatbuffers::VOffsetT = 16;
    pub const VT_REMOTE_BALANCE: flatbuffers::VOffsetT = 18;
    pub const VT_UNSETTLED_BALANCE: flatbuffers::VOffsetT = 20;
    pub const VT_LOCAL_CHANNEL_SETTINGS: flatbuffers::VOffsetT = 22;
    pub const VT_REMOTE_CHANNEL_SETTINGS: flatbuffers::VOffsetT = 24;

  #[inline]
  pub fn short_channel_id(&self) -> u64 {
    self._tab.get::<u64>(ChannelState::VT_SHORT_CHANNEL_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn local_node_id(&self) -> Option<&'a PubKey> {
    self._tab.get::<PubKey>(ChannelState::VT_LOCAL_NODE_ID, None)
  }
  #[inline]
  pub fn remote_node_id(&self) -> Option<&'a PubKey> {
    self._tab.get::<PubKey>(ChannelState::VT_REMOTE_NODE_ID, None)
  }
  #[inline]
  pub fn active(&self) -> bool {
    self._tab.get::<bool>(ChannelState::VT_ACTIVE, Some(true)).unwrap()
  }
  #[inline]
  pub fn private(&self) -> bool {
    self._tab.get::<bool>(ChannelState::VT_PRIVATE, Some(false)).unwrap()
  }
  #[inline]
  pub fn capacity(&self) -> u64 {
    self._tab.get::<u64>(ChannelState::VT_CAPACITY, Some(0)).unwrap()
  }
  #[inline]
  pub fn local_balance(&self) -> u64 {
    self._tab.get::<u64>(ChannelState::VT_LOCAL_BALANCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn remote_balance(&self) -> u64 {
    self._tab.get::<u64>(ChannelState::VT_REMOTE_BALANCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn unsettled_balance(&self) -> u64 {
    self._tab.get::<u64>(ChannelState::VT_UNSETTLED_BALANCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn local_channel_settings(&self) -> Option<ChannelSettings<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<ChannelSettings>>(ChannelState::VT_LOCAL_CHANNEL_SETTINGS, None)
  }
  #[inline]
  pub fn remote_channel_settings(&self) -> Option<ChannelSettings<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<ChannelSettings>>(ChannelState::VT_REMOTE_CHANNEL_SETTINGS, None)
  }
}

impl flatbuffers::Verifiable for ChannelState<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>(&"short_channel_id", Self::VT_SHORT_CHANNEL_ID, false)?
     .visit_field::<PubKey>(&"local_node_id", Self::VT_LOCAL_NODE_ID, false)?
     .visit_field::<PubKey>(&"remote_node_id", Self::VT_REMOTE_NODE_ID, false)?
     .visit_field::<bool>(&"active", Self::VT_ACTIVE, false)?
     .visit_field::<bool>(&"private", Self::VT_PRIVATE, false)?
     .visit_field::<u64>(&"capacity", Self::VT_CAPACITY, false)?
     .visit_field::<u64>(&"local_balance", Self::VT_LOCAL_BALANCE, false)?
     .visit_field::<u64>(&"remote_balance", Self::VT_REMOTE_BALANCE, false)?
     .visit_field::<u64>(&"unsettled_balance", Self::VT_UNSETTLED_BALANCE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<ChannelSettings>>(&"local_channel_settings", Self::VT_LOCAL_CHANNEL_SETTINGS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<ChannelSettings>>(&"remote_channel_settings", Self::VT_REMOTE_CHANNEL_SETTINGS, false)?
     .finish();
    Ok(())
  }
}
pub struct ChannelStateArgs<'a> {
    pub short_channel_id: u64,
    pub local_node_id: Option<&'a PubKey>,
    pub remote_node_id: Option<&'a PubKey>,
    pub active: bool,
    pub private: bool,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub unsettled_balance: u64,
    pub local_channel_settings: Option<flatbuffers::WIPOffset<ChannelSettings<'a>>>,
    pub remote_channel_settings: Option<flatbuffers::WIPOffset<ChannelSettings<'a>>>,
}
impl<'a> Default for ChannelStateArgs<'a> {
    #[inline]
    fn default() -> Self {
        ChannelStateArgs {
            short_channel_id: 0,
            local_node_id: None,
            remote_node_id: None,
            active: true,
            private: false,
            capacity: 0,
            local_balance: 0,
            remote_balance: 0,
            unsettled_balance: 0,
            local_channel_settings: None,
            remote_channel_settings: None,
        }
    }
}
pub struct ChannelStateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ChannelStateBuilder<'a, 'b> {
  #[inline]
  pub fn add_short_channel_id(&mut self, short_channel_id: u64) {
    self.fbb_.push_slot::<u64>(ChannelState::VT_SHORT_CHANNEL_ID, short_channel_id, 0);
  }
  #[inline]
  pub fn add_local_node_id(&mut self, local_node_id: &PubKey) {
    self.fbb_.push_slot_always::<&PubKey>(ChannelState::VT_LOCAL_NODE_ID, local_node_id);
  }
  #[inline]
  pub fn add_remote_node_id(&mut self, remote_node_id: &PubKey) {
    self.fbb_.push_slot_always::<&PubKey>(ChannelState::VT_REMOTE_NODE_ID, remote_node_id);
  }
  #[inline]
  pub fn add_active(&mut self, active: bool) {
    self.fbb_.push_slot::<bool>(ChannelState::VT_ACTIVE, active, true);
  }
  #[inline]
  pub fn add_private(&mut self, private: bool) {
    self.fbb_.push_slot::<bool>(ChannelState::VT_PRIVATE, private, false);
  }
  #[inline]
  pub fn add_capacity(&mut self, capacity: u64) {
    self.fbb_.push_slot::<u64>(ChannelState::VT_CAPACITY, capacity, 0);
  }
  #[inline]
  pub fn add_local_balance(&mut self, local_balance: u64) {
    self.fbb_.push_slot::<u64>(ChannelState::VT_LOCAL_BALANCE, local_balance, 0);
  }
  #[inline]
  pub fn add_remote_balance(&mut self, remote_balance: u64) {
    self.fbb_.push_slot::<u64>(ChannelState::VT_REMOTE_BALANCE, remote_balance, 0);
  }
  #[inline]
  pub fn add_unsettled_balance(&mut self, unsettled_balance: u64) {
    self.fbb_.push_slot::<u64>(ChannelState::VT_UNSETTLED_BALANCE, unsettled_balance, 0);
  }
  #[inline]
  pub fn add_local_channel_settings(&mut self, local_channel_settings: flatbuffers::WIPOffset<ChannelSettings<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ChannelSettings>>(ChannelState::VT_LOCAL_CHANNEL_SETTINGS, local_channel_settings);
  }
  #[inline]
  pub fn add_remote_channel_settings(&mut self, remote_channel_settings: flatbuffers::WIPOffset<ChannelSettings<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ChannelSettings>>(ChannelState::VT_REMOTE_CHANNEL_SETTINGS, remote_channel_settings);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ChannelStateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ChannelStateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ChannelState<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ChannelState<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ChannelState");
      ds.field("short_channel_id", &self.short_channel_id());
      ds.field("local_node_id", &self.local_node_id());
      ds.field("remote_node_id", &self.remote_node_id());
      ds.field("active", &self.active());
      ds.field("private", &self.private());
      ds.field("capacity", &self.capacity());
      ds.field("local_balance", &self.local_balance());
      ds.field("remote_balance", &self.remote_balance());
      ds.field("unsettled_balance", &self.unsettled_balance());
      ds.field("local_channel_settings", &self.local_channel_settings());
      ds.field("remote_channel_settings", &self.remote_channel_settings());
      ds.finish()
  }
}
pub enum ChannelScrapeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ChannelScrape<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ChannelScrape<'a> {
    type Inner = ChannelScrape<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> ChannelScrape<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ChannelScrape { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ChannelScrapeArgs<'args>) -> flatbuffers::WIPOffset<ChannelScrape<'bldr>> {
      let mut builder = ChannelScrapeBuilder::new(_fbb);
      builder.add_scrape_timestamp(args.scrape_timestamp);
      if let Some(x) = args.state { builder.add_state(x); }
      builder.finish()
    }

    pub const VT_SCRAPE_TIMESTAMP: flatbuffers::VOffsetT = 4;
    pub const VT_STATE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn scrape_timestamp(&self) -> u64 {
    self._tab.get::<u64>(ChannelScrape::VT_SCRAPE_TIMESTAMP, Some(0)).unwrap()
  }
  #[inline]
  pub fn state(&self) -> Option<ChannelState<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<ChannelState>>(ChannelScrape::VT_STATE, None)
  }
}

impl flatbuffers::Verifiable for ChannelScrape<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>(&"scrape_timestamp", Self::VT_SCRAPE_TIMESTAMP, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<ChannelState>>(&"state", Self::VT_STATE, false)?
     .finish();
    Ok(())
  }
}
pub struct ChannelScrapeArgs<'a> {
    pub scrape_timestamp: u64,
    pub state: Option<flatbuffers::WIPOffset<ChannelState<'a>>>,
}
impl<'a> Default for ChannelScrapeArgs<'a> {
    #[inline]
    fn default() -> Self {
        ChannelScrapeArgs {
            scrape_timestamp: 0,
            state: None,
        }
    }
}
pub struct ChannelScrapeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ChannelScrapeBuilder<'a, 'b> {
  #[inline]
  pub fn add_scrape_timestamp(&mut self, scrape_timestamp: u64) {
    self.fbb_.push_slot::<u64>(ChannelScrape::VT_SCRAPE_TIMESTAMP, scrape_timestamp, 0);
  }
  #[inline]
  pub fn add_state(&mut self, state: flatbuffers::WIPOffset<ChannelState<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ChannelState>>(ChannelScrape::VT_STATE, state);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ChannelScrapeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ChannelScrapeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ChannelScrape<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ChannelScrape<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ChannelScrape");
      ds.field("scrape_timestamp", &self.scrape_timestamp());
      ds.field("state", &self.state());
      ds.finish()
  }
}
