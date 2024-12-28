#[allow(unused)]
#[derive(Debug)]
pub struct ApiResponse {
    status_code: u16,
    msg: Option<String>,
    reqwest_error: Option<reqwest::Error>
}

#[allow(unused)]
impl ApiResponse {
    pub fn from(status_code: u16, msg: Option<String>, reqwest_error: Option<reqwest::Error>) -> ApiResponse {
        ApiResponse {
            status_code,
            msg,
            reqwest_error
        }
    }

    pub fn default() -> ApiResponse {
        ApiResponse {
            status_code: 0,
            msg: None,
            reqwest_error: None
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code.clone()
    }

    pub fn msg(&self) -> Option<String> {
        self.msg.clone()
    }

    pub fn reqwest_error(&self) -> &Option<reqwest::Error> {
        &self.reqwest_error
    }

    pub fn is_err(&self) -> bool {
        self.status_code >= 400
    }
}