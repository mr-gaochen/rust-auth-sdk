use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub amount: Option<f64>,
    pub area: Option<String>,
    pub avatar: Option<String>,
    pub birthday: Option<i64>,
    pub city: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    pub district: Option<String>,
    pub email: Option<String>,
    pub gender: i64,
    pub intro: Option<String>,
    #[serde(rename = "isStop")]
    pub is_stop: bool,
    pub name: String,
    pub phone: Option<String>,
    #[serde(rename = "projectCode")]
    pub project_code: String,
    pub province: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userNo")]
    pub user_no: Option<String>,
    #[serde(rename = "userRole")]
    pub user_role: String,
    #[serde(rename = "userType")]
    pub user_type: String,
    #[serde(rename = "vipType")]
    pub vip_type: Option<String>,
}
