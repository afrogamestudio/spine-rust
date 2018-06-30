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
    slot_index: usize,
    frames: Vec<f64>,
    attachment_names: Vec<String>
}

pub struct EventTimeline
{

}

pub struct DrawOrderTimeline
{

}
