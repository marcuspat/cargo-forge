use std::collections::HashMap;
use anyhow::Result;

/// Mock structure to represent auth feature template generation
struct AuthFeatureGenerator {
    project_name: String,
    auth_types: Vec<String>, // jwt, oauth, basic
    include_middleware: bool,
    include_password_hashing: bool,
}

impl AuthFeatureGenerator {
    fn new(project_name: &str, auth_types: Vec<String>) -> Self {
        Self {
            project_name: project_name.to_string(),
            auth_types,
            include_middleware: true,
            include_password_hashing: true,
        }
    }
    
    fn generate_jwt_module(&self) -> String {
        r#"use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // subject (user id)
    pub exp: i64,     // expiry time
    pub iat: i64,     // issued at
    pub email: String,
    pub roles: Vec<String>,
}

impl Claims {
    pub fn new(user_id: String, email: String, roles: Vec<String>) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp();
        
        Self {
            sub: user_id,
            exp,
            iat: now.timestamp(),
            email,
            roles,
        }
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let secret = env::var("JWT_SECRET")?;
        
        Ok(Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        })
    }
    
    pub fn generate_token(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), claims, &self.encoding_key)
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_generation_and_verification() {
        std::env::set_var("JWT_SECRET", "test_secret_key_for_testing_only");
        
        let jwt_manager = JwtManager::new().unwrap();
        let claims = Claims::new(
            "user123".to_string(),
            "user@example.com".to_string(),
            vec!["user".to_string(), "admin".to_string()]
        );
        
        let token = jwt_manager.generate_token(&claims).unwrap();
        assert!(!token.is_empty());
        
        let verified_claims = jwt_manager.verify_token(&token).unwrap();
        assert_eq!(verified_claims.sub, "user123");
        assert_eq!(verified_claims.email, "user@example.com");
        assert_eq!(verified_claims.roles, vec!["user", "admin"]);
    }
}"#.to_string()
    }
    
    fn generate_oauth_module(&self) -> String {
        r#"use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, 
    RevocationUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
    reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

impl OAuthConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client_id: env::var("OAUTH_CLIENT_ID")?,
            client_secret: env::var("OAUTH_CLIENT_SECRET")?,
            auth_url: env::var("OAUTH_AUTH_URL")?,
            token_url: env::var("OAUTH_TOKEN_URL")?,
            redirect_url: env::var("OAUTH_REDIRECT_URL")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

pub struct OAuthClient {
    client: BasicClient,
}

impl OAuthClient {
    pub fn new(config: OAuthConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let client = BasicClient::new(
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
            AuthUrl::new(config.auth_url)?,
            Some(TokenUrl::new(config.token_url)?),
        )
        .set_redirect_uri(RedirectUrl::new(config.redirect_url)?);
        
        Ok(Self { client })
    }
    
    pub fn get_authorization_url(&self) -> (String, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url()
    }
    
    pub async fn exchange_code(&self, code: String) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await?;
        
        Ok(token.access_token().secret().to_string())
    }
}

// Provider-specific implementations
pub mod providers {
    use super::*;
    
    pub fn google_config() -> OAuthConfig {
        OAuthConfig {
            client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            redirect_url: env::var("GOOGLE_REDIRECT_URL").unwrap_or_default(),
        }
    }
    
    pub fn github_config() -> OAuthConfig {
        OAuthConfig {
            client_id: env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
            client_secret: env::var("GITHUB_CLIENT_SECRET").unwrap_or_default(),
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            redirect_url: env::var("GITHUB_REDIRECT_URL").unwrap_or_default(),
        }
    }
}"#.to_string()
    }
    
    fn generate_password_module(&self) -> String {
        r#"use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct PasswordManager {
    argon2: Argon2<'static>,
}

impl PasswordManager {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
    
    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hashing_and_verification() {
        let password_manager = PasswordManager::new();
        let password = "secure_password123!";
        
        let hash = password_manager.hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password);
        
        // Verify correct password
        assert!(password_manager.verify_password(password, &hash).unwrap());
        
        // Verify incorrect password
        assert!(!password_manager.verify_password("wrong_password", &hash).unwrap());
    }
}"#.to_string()
    }
    
    fn generate_auth_middleware(&self) -> String {
        r#"use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::auth::jwt::{JwtManager, Claims};

pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| AuthError::MissingToken)?;
        
        let jwt_manager = JwtManager::new()
            .map_err(|_| AuthError::InvalidToken)?;
        
        let claims = jwt_manager.verify_token(bearer.token())
            .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(AuthenticatedUser {
            user_id: claims.sub,
            email: claims.email,
            roles: claims.roles,
        })
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
        };
        
        let body = Json(json!({
            "error": error_message,
        }));
        
        (status, body).into_response()
    }
}

/// Middleware to check if user has required role
pub struct RequireRole(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for RequireRole
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthenticatedUser::from_request_parts(parts, state).await?;
        
        // Check if user has admin role
        if !user.roles.contains(&"admin".to_string()) {
            return Err(AuthError::InsufficientPermissions);
        }
        
        Ok(RequireRole("admin".to_string()))
    }
}"#.to_string()
    }
    
    fn generate_auth_routes(&self) -> String {
        r#"use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::{
    jwt::{JwtManager, Claims},
    password::PasswordManager,
    oauth::{OAuthClient, OAuthConfig},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}

pub struct AuthState {
    pub jwt_manager: Arc<JwtManager>,
    pub password_manager: Arc<PasswordManager>,
    pub oauth_client: Arc<OAuthClient>,
}

pub fn auth_routes() -> Router<AuthState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/oauth/google", get(oauth_google_redirect))
        .route("/auth/oauth/google/callback", get(oauth_google_callback))
        .route("/auth/logout", post(logout))
        .route("/auth/refresh", post(refresh_token))
}

async fn login(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    // TODO: Verify user credentials from database
    // For now, this is a mock implementation
    
    let claims = Claims::new(
        "user123".to_string(),
        payload.email,
        vec!["user".to_string()],
    );
    
    let token = state.jwt_manager.generate_token(&claims)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 86400, // 24 hours
    }))
}

async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    // Hash the password
    let password_hash = state.password_manager.hash_password(&payload.password)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Save user to database
    
    // Generate token for immediate login
    let claims = Claims::new(
        "new_user_id".to_string(),
        payload.email,
        vec!["user".to_string()],
    );
    
    let token = state.jwt_manager.generate_token(&claims)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 86400,
    }))
}

async fn oauth_google_redirect(State(state): State<AuthState>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state.oauth_client.get_authorization_url();
    Redirect::to(&auth_url)
}

async fn oauth_google_callback(
    State(state): State<AuthState>,
    Query(params): Query<OAuthCallback>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    // Exchange code for token
    let access_token = state.oauth_client.exchange_code(params.code)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Fetch user info from OAuth provider
    // TODO: Create or update user in database
    
    // Generate JWT token
    let claims = Claims::new(
        "oauth_user_id".to_string(),
        "oauth_user@example.com".to_string(),
        vec!["user".to_string()],
    );
    
    let token = state.jwt_manager.generate_token(&claims)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 86400,
    }))
}

async fn logout() -> impl IntoResponse {
    // TODO: Implement token revocation if needed
    Json(serde_json::json!({ "message": "Logged out successfully" }))
}

async fn refresh_token(
    State(state): State<AuthState>,
    // TODO: Accept refresh token
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    // TODO: Implement refresh token logic
    Err((axum::http::StatusCode::NOT_IMPLEMENTED, "Refresh token not implemented".to_string()))
}"#.to_string()
    }
    
    fn generate_file_structure(&self) -> Vec<(String, String)> {
        let mut files = vec![];
        
        // Base auth module
        files.push((
            "src/auth/mod.rs".to_string(),
            self.generate_auth_mod()
        ));
        
        // JWT support
        if self.auth_types.contains(&"jwt".to_string()) {
            files.push((
                "src/auth/jwt.rs".to_string(),
                self.generate_jwt_module()
            ));
        }
        
        // OAuth support
        if self.auth_types.contains(&"oauth".to_string()) {
            files.push((
                "src/auth/oauth.rs".to_string(),
                self.generate_oauth_module()
            ));
        }
        
        // Password hashing
        if self.include_password_hashing {
            files.push((
                "src/auth/password.rs".to_string(),
                self.generate_password_module()
            ));
        }
        
        // Middleware
        if self.include_middleware {
            files.push((
                "src/auth/middleware.rs".to_string(),
                self.generate_auth_middleware()
            ));
        }
        
        // Auth routes
        files.push((
            "src/auth/routes.rs".to_string(),
            self.generate_auth_routes()
        ));
        
        // Environment variables example
        files.push((
            ".env.auth.example".to_string(),
            self.generate_env_example()
        ));
        
        files
    }
    
    fn generate_auth_mod(&self) -> String {
        let mut modules = vec![];
        
        if self.auth_types.contains(&"jwt".to_string()) {
            modules.push("pub mod jwt;");
        }
        if self.auth_types.contains(&"oauth".to_string()) {
            modules.push("pub mod oauth;");
        }
        if self.include_password_hashing {
            modules.push("pub mod password;");
        }
        if self.include_middleware {
            modules.push("pub mod middleware;");
        }
        modules.push("pub mod routes;");
        
        modules.join("\n")
    }
    
    fn generate_env_example(&self) -> String {
        let mut env_vars = vec![];
        
        if self.auth_types.contains(&"jwt".to_string()) {
            env_vars.push("# JWT Configuration");
            env_vars.push("JWT_SECRET=your-secret-key-here-min-32-chars");
            env_vars.push("JWT_EXPIRATION=86400");
            env_vars.push("");
        }
        
        if self.auth_types.contains(&"oauth".to_string()) {
            env_vars.push("# OAuth Configuration");
            env_vars.push("# Google OAuth");
            env_vars.push("GOOGLE_CLIENT_ID=your-google-client-id");
            env_vars.push("GOOGLE_CLIENT_SECRET=your-google-client-secret");
            env_vars.push("GOOGLE_REDIRECT_URL=http://localhost:3000/auth/oauth/google/callback");
            env_vars.push("");
            env_vars.push("# GitHub OAuth");
            env_vars.push("GITHUB_CLIENT_ID=your-github-client-id");
            env_vars.push("GITHUB_CLIENT_SECRET=your-github-client-secret"); 
            env_vars.push("GITHUB_REDIRECT_URL=http://localhost:3000/auth/oauth/github/callback");
        }
        
        env_vars.join("\n")
    }
    
    fn get_required_dependencies(&self) -> HashMap<String, String> {
        let mut deps = HashMap::new();
        
        if self.auth_types.contains(&"jwt".to_string()) {
            deps.insert("jsonwebtoken".to_string(), "\"9\"".to_string());
        }
        
        if self.auth_types.contains(&"oauth".to_string()) {
            deps.insert("oauth2".to_string(), "\"4\"".to_string());
            deps.insert("reqwest".to_string(), r#"{ version = "0.11", features = ["json"] }"#.to_string());
        }
        
        if self.include_password_hashing {
            deps.insert("argon2".to_string(), "\"0.5\"".to_string());
        }
        
        // Common auth dependencies
        deps.insert("serde".to_string(), r#"{ version = "1", features = ["derive"] }"#.to_string());
        deps.insert("serde_json".to_string(), "\"1\"".to_string());
        deps.insert("chrono".to_string(), r#"{ version = "0.4", features = ["serde"] }"#.to_string());
        
        deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_module_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string()]);
        let jwt_module = generator.generate_jwt_module();
        
        assert!(jwt_module.contains("use jsonwebtoken"));
        assert!(jwt_module.contains("pub struct Claims"));
        assert!(jwt_module.contains("pub struct JwtManager"));
        assert!(jwt_module.contains("generate_token"));
        assert!(jwt_module.contains("verify_token"));
    }
    
    #[test]
    fn test_oauth_module_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["oauth".to_string()]);
        let oauth_module = generator.generate_oauth_module();
        
        assert!(oauth_module.contains("use oauth2"));
        assert!(oauth_module.contains("pub struct OAuthConfig"));
        assert!(oauth_module.contains("pub struct OAuthClient"));
        assert!(oauth_module.contains("get_authorization_url"));
        assert!(oauth_module.contains("exchange_code"));
        assert!(oauth_module.contains("google_config"));
        assert!(oauth_module.contains("github_config"));
    }
    
    #[test]
    fn test_password_module_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec![]);
        let password_module = generator.generate_password_module();
        
        assert!(password_module.contains("use argon2"));
        assert!(password_module.contains("pub struct PasswordManager"));
        assert!(password_module.contains("hash_password"));
        assert!(password_module.contains("verify_password"));
    }
    
    #[test]
    fn test_auth_middleware_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string()]);
        let middleware = generator.generate_auth_middleware();
        
        assert!(middleware.contains("pub struct AuthenticatedUser"));
        assert!(middleware.contains("FromRequestParts"));
        assert!(middleware.contains("pub enum AuthError"));
        assert!(middleware.contains("pub struct RequireRole"));
    }
    
    #[test]
    fn test_auth_routes_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string(), "oauth".to_string()]);
        let routes = generator.generate_auth_routes();
        
        assert!(routes.contains("/auth/login"));
        assert!(routes.contains("/auth/register"));
        assert!(routes.contains("/auth/oauth/google"));
        assert!(routes.contains("/auth/logout"));
        assert!(routes.contains("LoginRequest"));
        assert!(routes.contains("LoginResponse"));
    }
    
    #[test]
    fn test_file_structure_with_jwt_only() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string()]);
        let files = generator.generate_file_structure();
        let file_paths: Vec<String> = files.iter().map(|(path, _)| path.clone()).collect();
        
        assert!(file_paths.contains(&"src/auth/mod.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/jwt.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/password.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/middleware.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/routes.rs".to_string()));
        assert!(!file_paths.contains(&"src/auth/oauth.rs".to_string()));
    }
    
    #[test]
    fn test_file_structure_with_oauth_only() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["oauth".to_string()]);
        let files = generator.generate_file_structure();
        let file_paths: Vec<String> = files.iter().map(|(path, _)| path.clone()).collect();
        
        assert!(file_paths.contains(&"src/auth/oauth.rs".to_string()));
        assert!(!file_paths.contains(&"src/auth/jwt.rs".to_string()));
    }
    
    #[test]
    fn test_file_structure_with_all_auth_types() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string(), "oauth".to_string(), "basic".to_string()]);
        let files = generator.generate_file_structure();
        let file_paths: Vec<String> = files.iter().map(|(path, _)| path.clone()).collect();
        
        assert!(file_paths.contains(&"src/auth/jwt.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/oauth.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/password.rs".to_string()));
        assert!(file_paths.contains(&"src/auth/middleware.rs".to_string()));
    }
    
    #[test]
    fn test_env_example_generation() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string(), "oauth".to_string()]);
        let env_example = generator.generate_env_example();
        
        assert!(env_example.contains("JWT_SECRET"));
        assert!(env_example.contains("GOOGLE_CLIENT_ID"));
        assert!(env_example.contains("GITHUB_CLIENT_ID"));
    }
    
    #[test]
    fn test_required_dependencies_jwt() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string()]);
        let deps = generator.get_required_dependencies();
        
        assert!(deps.contains_key("jsonwebtoken"));
        assert!(deps.contains_key("chrono"));
        assert!(deps.contains_key("serde"));
        assert!(!deps.contains_key("oauth2"));
    }
    
    #[test]
    fn test_required_dependencies_oauth() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["oauth".to_string()]);
        let deps = generator.get_required_dependencies();
        
        assert!(deps.contains_key("oauth2"));
        assert!(deps.contains_key("reqwest"));
        assert!(!deps.contains_key("jsonwebtoken"));
    }
    
    #[test]
    fn test_required_dependencies_all_features() {
        let generator = AuthFeatureGenerator::new("my_app", vec!["jwt".to_string(), "oauth".to_string()]);
        let deps = generator.get_required_dependencies();
        
        assert!(deps.contains_key("jsonwebtoken"));
        assert!(deps.contains_key("oauth2"));
        assert!(deps.contains_key("argon2"));
        assert!(deps.contains_key("serde"));
        assert!(deps.contains_key("chrono"));
    }
}