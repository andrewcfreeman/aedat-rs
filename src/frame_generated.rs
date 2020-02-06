// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FrameFormat {
    Gray = 0,
    Bgr = 16,
    Bgra = 24,
}

const ENUM_MIN_FRAME_FORMAT: i8 = 0;
const ENUM_MAX_FRAME_FORMAT: i8 = 24;

impl<'a> flatbuffers::Follow<'a> for FrameFormat {
    type Inner = Self;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::read_scalar_at::<Self>(buf, loc)
    }
}

impl flatbuffers::EndianScalar for FrameFormat {
    #[inline]
    fn to_little_endian(self) -> Self {
        let n = i8::to_le(self as i8);
        let p = &n as *const i8 as *const FrameFormat;
        unsafe { *p }
    }
    #[inline]
    fn from_little_endian(self) -> Self {
        let n = i8::from_le(self as i8);
        let p = &n as *const i8 as *const FrameFormat;
        unsafe { *p }
    }
}

impl flatbuffers::Push for FrameFormat {
    type Output = FrameFormat;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<FrameFormat>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_FRAME_FORMAT: [FrameFormat; 3] =
    [FrameFormat::Gray, FrameFormat::Bgr, FrameFormat::Bgra];

pub enum FrameOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Frame<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Frame<'a> {
    type Inner = Frame<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Frame<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Frame { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FrameArgs<'args>,
    ) -> flatbuffers::WIPOffset<Frame<'bldr>> {
        let mut builder = FrameBuilder::new(_fbb);
        builder.add_exposure_end_t(args.exposure_end_t);
        builder.add_exposure_begin_t(args.exposure_begin_t);
        builder.add_end_t(args.end_t);
        builder.add_begin_t(args.begin_t);
        builder.add_t(args.t);
        if let Some(x) = args.pixels {
            builder.add_pixels(x);
        }
        builder.add_offset_y(args.offset_y);
        builder.add_offset_x(args.offset_x);
        builder.add_height(args.height);
        builder.add_width(args.width);
        builder.add_format(args.format);
        builder.finish()
    }

    pub const VT_T: flatbuffers::VOffsetT = 4;
    pub const VT_BEGIN_T: flatbuffers::VOffsetT = 6;
    pub const VT_END_T: flatbuffers::VOffsetT = 8;
    pub const VT_EXPOSURE_BEGIN_T: flatbuffers::VOffsetT = 10;
    pub const VT_EXPOSURE_END_T: flatbuffers::VOffsetT = 12;
    pub const VT_FORMAT: flatbuffers::VOffsetT = 14;
    pub const VT_WIDTH: flatbuffers::VOffsetT = 16;
    pub const VT_HEIGHT: flatbuffers::VOffsetT = 18;
    pub const VT_OFFSET_X: flatbuffers::VOffsetT = 20;
    pub const VT_OFFSET_Y: flatbuffers::VOffsetT = 22;
    pub const VT_PIXELS: flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn t(&self) -> i64 {
        self._tab.get::<i64>(Frame::VT_T, Some(0)).unwrap()
    }
    #[inline]
    pub fn begin_t(&self) -> i64 {
        self._tab.get::<i64>(Frame::VT_BEGIN_T, Some(0)).unwrap()
    }
    #[inline]
    pub fn end_t(&self) -> i64 {
        self._tab.get::<i64>(Frame::VT_END_T, Some(0)).unwrap()
    }
    #[inline]
    pub fn exposure_begin_t(&self) -> i64 {
        self._tab
            .get::<i64>(Frame::VT_EXPOSURE_BEGIN_T, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn exposure_end_t(&self) -> i64 {
        self._tab
            .get::<i64>(Frame::VT_EXPOSURE_END_T, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn format(&self) -> FrameFormat {
        self._tab
            .get::<FrameFormat>(Frame::VT_FORMAT, Some(FrameFormat::Gray))
            .unwrap()
    }
    #[inline]
    pub fn width(&self) -> i16 {
        self._tab.get::<i16>(Frame::VT_WIDTH, Some(0)).unwrap()
    }
    #[inline]
    pub fn height(&self) -> i16 {
        self._tab.get::<i16>(Frame::VT_HEIGHT, Some(0)).unwrap()
    }
    #[inline]
    pub fn offset_x(&self) -> i16 {
        self._tab.get::<i16>(Frame::VT_OFFSET_X, Some(0)).unwrap()
    }
    #[inline]
    pub fn offset_y(&self) -> i16 {
        self._tab.get::<i16>(Frame::VT_OFFSET_Y, Some(0)).unwrap()
    }
    #[inline]
    pub fn pixels(&self) -> Option<&'a [u8]> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                Frame::VT_PIXELS,
                None,
            )
            .map(|v| v.safe_slice())
    }
}

pub struct FrameArgs<'a> {
    pub t: i64,
    pub begin_t: i64,
    pub end_t: i64,
    pub exposure_begin_t: i64,
    pub exposure_end_t: i64,
    pub format: FrameFormat,
    pub width: i16,
    pub height: i16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub pixels: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for FrameArgs<'a> {
    #[inline]
    fn default() -> Self {
        FrameArgs {
            t: 0,
            begin_t: 0,
            end_t: 0,
            exposure_begin_t: 0,
            exposure_end_t: 0,
            format: FrameFormat::Gray,
            width: 0,
            height: 0,
            offset_x: 0,
            offset_y: 0,
            pixels: None,
        }
    }
}
pub struct FrameBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FrameBuilder<'a, 'b> {
    #[inline]
    pub fn add_t(&mut self, t: i64) {
        self.fbb_.push_slot::<i64>(Frame::VT_T, t, 0);
    }
    #[inline]
    pub fn add_begin_t(&mut self, begin_t: i64) {
        self.fbb_.push_slot::<i64>(Frame::VT_BEGIN_T, begin_t, 0);
    }
    #[inline]
    pub fn add_end_t(&mut self, end_t: i64) {
        self.fbb_.push_slot::<i64>(Frame::VT_END_T, end_t, 0);
    }
    #[inline]
    pub fn add_exposure_begin_t(&mut self, exposure_begin_t: i64) {
        self.fbb_
            .push_slot::<i64>(Frame::VT_EXPOSURE_BEGIN_T, exposure_begin_t, 0);
    }
    #[inline]
    pub fn add_exposure_end_t(&mut self, exposure_end_t: i64) {
        self.fbb_
            .push_slot::<i64>(Frame::VT_EXPOSURE_END_T, exposure_end_t, 0);
    }
    #[inline]
    pub fn add_format(&mut self, format: FrameFormat) {
        self.fbb_
            .push_slot::<FrameFormat>(Frame::VT_FORMAT, format, FrameFormat::Gray);
    }
    #[inline]
    pub fn add_width(&mut self, width: i16) {
        self.fbb_.push_slot::<i16>(Frame::VT_WIDTH, width, 0);
    }
    #[inline]
    pub fn add_height(&mut self, height: i16) {
        self.fbb_.push_slot::<i16>(Frame::VT_HEIGHT, height, 0);
    }
    #[inline]
    pub fn add_offset_x(&mut self, offset_x: i16) {
        self.fbb_.push_slot::<i16>(Frame::VT_OFFSET_X, offset_x, 0);
    }
    #[inline]
    pub fn add_offset_y(&mut self, offset_y: i16) {
        self.fbb_.push_slot::<i16>(Frame::VT_OFFSET_Y, offset_y, 0);
    }
    #[inline]
    pub fn add_pixels(&mut self, pixels: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Frame::VT_PIXELS, pixels);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FrameBuilder<'a, 'b> {
        let start = _fbb.start_table();
        FrameBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Frame<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

#[inline]
pub fn get_root_as_frame<'a>(buf: &'a [u8]) -> Frame<'a> {
    flatbuffers::get_root::<Frame<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_frame<'a>(buf: &'a [u8]) -> Frame<'a> {
    flatbuffers::get_size_prefixed_root::<Frame<'a>>(buf)
}

pub const FRAME_IDENTIFIER: &'static str = "FRME";

#[inline]
pub fn frame_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, FRAME_IDENTIFIER, false);
}

#[inline]
pub fn frame_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, FRAME_IDENTIFIER, true);
}

#[inline]
pub fn finish_frame_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Frame<'a>>,
) {
    fbb.finish(root, Some(FRAME_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_frame_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Frame<'a>>,
) {
    fbb.finish_size_prefixed(root, Some(FRAME_IDENTIFIER));
}
