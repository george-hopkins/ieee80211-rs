use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

use crate::common::{
    subtypes::ManagementFrameSubtype, FCFFlags, FrameControlField, FrameType,
};

use self::{action::ActionFrameBody, beacon::BeaconFrameBody, header::ManagementFrameHeader};

pub mod action;
pub mod beacon;
pub mod header;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManagementFrameBody<'a> {
    Action(ActionFrameBody<'a>),
    ActionNoAck(ActionFrameBody<'a>),
    Beacon(BeaconFrameBody<'a>),
    ATIM,
}
impl ManagementFrameBody<'_> {
    pub const fn get_sub_type(&self) -> ManagementFrameSubtype {
        match self {
            Self::Action(_) => ManagementFrameSubtype::Action,
            Self::ActionNoAck(_) => ManagementFrameSubtype::ActionNoAck,
            Self::Beacon(_) => ManagementFrameSubtype::Beacon,
            Self::ATIM => ManagementFrameSubtype::ATIM,
        }
    }
    pub const fn length_in_bytes(&self) -> usize {
        match self {
            Self::Action(action) | Self::ActionNoAck(action) => action.length_in_bytes(),
            Self::Beacon(beacon) => beacon.length_in_bytes(),
            Self::ATIM => 0,
        }
    }
}
impl MeasureWith<()> for ManagementFrameBody<'_> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.length_in_bytes()
    }
}
impl<'a> TryFromCtx<'a, ManagementFrameSubtype> for ManagementFrameBody<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(
        from: &'a [u8],
        sub_type: ManagementFrameSubtype,
    ) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        Ok((
            match sub_type {
                ManagementFrameSubtype::Action => Self::Action(from.gread(&mut offset)?),
                ManagementFrameSubtype::ActionNoAck => Self::ActionNoAck(from.gread(&mut offset)?),
                ManagementFrameSubtype::Beacon => Self::Beacon(from.gread(&mut offset)?),
                ManagementFrameSubtype::ATIM => Self::ATIM,
                _ => {
                    return Err(scroll::Error::BadInput {
                        size: offset,
                        msg: "Management frame subtype not implemented.",
                    })
                }
            },
            offset,
        ))
    }
}
impl TryIntoCtx for ManagementFrameBody<'_> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        match self {
            Self::Action(action_frame_body) | Self::ActionNoAck(action_frame_body) => {
                buf.pwrite(action_frame_body, 0)
            }
            Self::Beacon(beacon_frame_body) => buf.pwrite(beacon_frame_body, 0),
            Self::ATIM => Ok(0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagementFrame<'a> {
    pub header: ManagementFrameHeader,
    pub body: ManagementFrameBody<'a>,
}
impl ManagementFrame<'_> {
    pub const fn get_fcf(&self) -> FrameControlField {
        FrameControlField {
            version: 0,
            frame_type: FrameType::Management(self.body.get_sub_type()),
            flags: self.header.fcf_flags,
        }
    }
    pub const fn length_in_bytes(&self) -> usize {
        self.header.length_in_bytes() + self.body.length_in_bytes()
    }
}
impl<'a> TryFromCtx<'a, (ManagementFrameSubtype, FCFFlags)> for ManagementFrame<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(
        from: &'a [u8],
        (subtype, fcf_flags): (ManagementFrameSubtype, FCFFlags),
    ) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let header = from.gread_with(&mut offset, fcf_flags)?;
        let body = from.gread_with(&mut offset, subtype)?;

        Ok((Self { header, body }, offset))
    }
}
impl TryIntoCtx for ManagementFrame<'_> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite(self.header, &mut offset)?;
        buf.gwrite(self.body, &mut offset)?;
        Ok(offset)
    }
}
