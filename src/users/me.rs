use actix_web::{HttpResponse,web};

use crate::utils::auth::Auth;
// use crate::jwt::VueToken;
use crate::models::{ResponseMsg,LoginUserInfo};

//token: web::Json<VueToken>

pub fn me_service(auth:Auth) -> HttpResponse {
	let user_info = LoginUserInfo{
		roles:vec!["admin".to_string()],
		introduction:Some("I am a super administrator".to_string()),
		avatar: Some("https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif".to_string()),
        name: "Super Admin".to_string(),
	};
    HttpResponse::Ok().json(ResponseMsg::new_ok(user_info))
}
