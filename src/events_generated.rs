// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

// struct Event, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    t_: i64,
    x_: i16,
    y_: i16,
    on_: bool,
    padding0__: u8,
    padding1__: u16,
} // pub struct Event
impl flatbuffers::SafeSliceAccess for Event {}
impl<'a> flatbuffers::Follow<'a> for Event {
    type Inner = &'a Event;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a Event>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a Event {
    type Inner = &'a Event;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<Event>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for Event {
    type Output = Event;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Event as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Event {
    type Output = Event;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Event as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl Event {
    pub fn new<'a>(_t: i64, _x: i16, _y: i16, _on: bool) -> Self {
        Event {
            t_: _t.to_little_endian(),
            x_: _x.to_little_endian(),
            y_: _y.to_little_endian(),
            on_: _on.to_little_endian(),

            padding0__: 0,
            padding1__: 0,
        }
    }
    pub fn t<'a>(&'a self) -> i64 {
        self.t_.from_little_endian()
    }
    pub fn x<'a>(&'a self) -> i16 {
        self.x_.from_little_endian()
    }
    pub fn y<'a>(&'a self) -> i16 {
        self.y_.from_little_endian()
    }
    pub fn on<'a>(&'a self) -> bool {
        self.on_.from_little_endian()
    }
}

pub enum EventPacketOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct EventPacket<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EventPacket<'a> {
    type Inner = EventPacket<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> EventPacket<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        EventPacket { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args EventPacketArgs<'args>,
    ) -> flatbuffers::WIPOffset<EventPacket<'bldr>> {
        let mut builder = EventPacketBuilder::new(_fbb);
        if let Some(x) = args.elements {
            builder.add_elements(x);
        }
        builder.finish()
    }

    pub const VT_ELEMENTS: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn elements(&self) -> Option<&'a [Event]> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<Event>>>(
                EventPacket::VT_ELEMENTS,
                None,
            )
            .map(|v| v.safe_slice())
    }
}

pub struct EventPacketArgs<'a> {
    pub elements: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Event>>>,
}
impl<'a> Default for EventPacketArgs<'a> {
    #[inline]
    fn default() -> Self {
        EventPacketArgs { elements: None }
    }
}
pub struct EventPacketBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EventPacketBuilder<'a, 'b> {
    #[inline]
    pub fn add_elements(
        &mut self,
        elements: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Event>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(EventPacket::VT_ELEMENTS, elements);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventPacketBuilder<'a, 'b> {
        let start = _fbb.start_table();
        EventPacketBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<EventPacket<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

#[inline]
pub fn get_root_as_event_packet<'a>(buf: &'a [u8]) -> EventPacket<'a> {
    flatbuffers::get_root::<EventPacket<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_event_packet<'a>(buf: &'a [u8]) -> EventPacket<'a> {
    flatbuffers::get_size_prefixed_root::<EventPacket<'a>>(buf)
}

pub const EVENT_PACKET_IDENTIFIER: &'static str = "EVTS";

#[inline]
pub fn event_packet_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, EVENT_PACKET_IDENTIFIER, false);
}

#[inline]
pub fn event_packet_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, EVENT_PACKET_IDENTIFIER, true);
}

#[inline]
pub fn finish_event_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<EventPacket<'a>>,
) {
    fbb.finish(root, Some(EVENT_PACKET_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_event_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<EventPacket<'a>>,
) {
    fbb.finish_size_prefixed(root, Some(EVENT_PACKET_IDENTIFIER));
}
