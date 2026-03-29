use crate::network::login::auth_type::AuthType;

pub enum AuthPayload {
    Chains(Vec<String>, AuthType),
    Token(String, AuthType)
}