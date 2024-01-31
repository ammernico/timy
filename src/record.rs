#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub _id: String,
    pub _username: String,
    pub date: String,
    pub _account: String,
    pub sub_account: String,
    pub _time: Option<f32>,
    pub _billed_time: Option<f32>,
    pub comment: String,
}
