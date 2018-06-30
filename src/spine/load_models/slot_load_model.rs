

#[derive(Serialize, Deserialize, Clone)]
pub struct SlotLoadModel
{
    pub name: String,
    pub bone: String,
    pub attachment: Option<String>,
    pub color: Option<String>
}
