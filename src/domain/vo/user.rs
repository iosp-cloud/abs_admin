use crate::domain::domain::{LoginCheck, SysUser};
use crate::domain::vo::SysRoleVO;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

///后台用户
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,

    pub roles: Vec<SysRoleVO>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            password: arg.password,
            name: arg.name,
            login_check: arg.login_check,
            del: arg.del,
            create_date: arg.create_date,
            roles: vec![],
        }
    }
}