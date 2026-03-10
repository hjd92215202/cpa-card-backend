use crate::AppState; 
use crate::error::AppError;
use crate::models::user::{User, RegisterRequest, LoginRequest, AuthResponse, Claims}; 
use crate::repository::user_repo::UserRepository;
use axum::{
    async_trait,
    extract::{FromRequestParts, State, FromRef},
    http::{request::Parts, StatusCode},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use validator::Validate;

/// 身份认证提取器：在其他处理器中直接通过参数 AuthUser(user_id) 获取当前用户
pub struct AuthUser(pub i32);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>, // 允许从通用状态 S 中提取出 AppState
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 获取 AppState
        let state = AppState::from_ref(state);
        
        // 2. 从 Header 提取 Token
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .filter(|s| s.starts_with("Bearer "));

        match auth_header {
            Some(header) => {
                let token = &header[7..];
                
                // 3. 使用 AppState 中的密钥进行验证，不再读取环境变量
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(state.jwt_secret.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(data) => Ok(AuthUser(data.claims.sub)),
                    Err(_) => Err((StatusCode::UNAUTHORIZED, "无效或过期的 Token".to_string())),
                }
            }
            None => Err((StatusCode::UNAUTHORIZED, "请先登录".to_string())),
        }
    }
}

/// 用户注册接口
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    // 1. 基础校验
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // 2. 检查用户名冲突
    if UserRepository::find_by_username(&state.db, &payload.username).await?.is_some() {
        return Err(AppError::BadRequest("该用户名已被注册".into()));
    }

    // 3. 密码加盐哈希（使用引用 & 以保留 payload 所有权）
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::BadRequest("内部安全模块异常".into()))?;

    // 4. 写入数据库
    let user = UserRepository::create(&state.db, payload, password_hash).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// 用户登录接口
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // 1. 查找用户
    let user = UserRepository::find_by_username(&state.db, &payload.username)
        .await?
        .ok_or_else(|| AppError::BadRequest("用户名或密码错误".into()))?;

    // 2. 验证密码（使用引用 &）
    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::BadRequest("认证服务故障".into()))?;

    if !valid {
        return Err(AppError::BadRequest("用户名或密码错误".into()));
    }

    // 3. 计算过期时间 (7 天)
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60 * 24 * 7;

    let claims = Claims { 
        sub: user.id, 
        exp: expiration as usize 
    };
    
    // 4. 签发 Token（直接从 state 获取密钥）
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    ).map_err(|_| AppError::BadRequest("Token 签发失败".into()))?;

    Ok(Json(AuthResponse { 
        token, 
        user_id: user.id, 
        username: user.username 
    }))
}