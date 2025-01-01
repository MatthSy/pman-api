#[allow(unused)]
pub fn gen_api_key() -> String {
    uuid::Uuid::new_v4().to_string()
}