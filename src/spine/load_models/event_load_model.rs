

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventLoadModel
{
	pub name: String,
	pub int: i32,
	pub float: f64,
	pub string: String
}
