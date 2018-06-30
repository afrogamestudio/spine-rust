pub enum Timeline
{
    Curve(CurveTimeline),
    Attachment(AttachmentTimeline),
    Event(EventTimeline),
    DrawOrder(DrawOrderTimeline)
}

pub struct CurveTimeline
{

}

pub struct AttachmentTimeline
{
    pub slot_index: usize,
    pub frames: Vec<f64>,
    pub attachment_names: Vec<String>
}

pub struct EventTimeline
{

}

pub struct DrawOrderTimeline
{

}
