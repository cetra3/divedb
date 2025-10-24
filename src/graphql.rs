use crate::activitypub::activities::CreatePost;
use crate::email::Emailer;
use crate::openid::OpenIDClient;
use crate::search::{SearchResult, Searcher};
use crate::{db::DbHandle, facebook::FacebookOauth, schema::*, subsurface, token::TokenEncryptor};
use crate::{SiteContext, SITE_URL};
use aes_gcm::Aes256Gcm;
use anyhow::{anyhow, Context as AnyhowContext};
use async_graphql::*;
use foyer::HybridCache;
use governor::clock::{Clock, DefaultClock};
use governor::state::keyed::DefaultKeyedStateStore;
use governor::{Quota, RateLimiter};
use reqwest::Client;
use scrypt::*;
use std::net::IpAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tracing::*;
use uuid::Uuid;

pub type IpLimiter = RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>;

pub struct SchemaContext {
    pub con: ConnectionContext,
    pub web: Arc<WebContext>,
    pub fed: activitypub_federation::config::Data<Arc<WebContext>>,
}

pub struct ConnectionContext {
    pub remote_ip: IpAddr,
    pub user: Option<User>,
}

pub struct WebContext {
    pub handle: DbHandle,
    pub rate_limiter: IpLimiter,
    pub cipher: Aes256Gcm,
    pub searcher: Searcher,
    pub emailer: Emailer,
    pub site_context: SiteContext,
    pub dive_batch: DiveSiteLoader,
    pub sealife_batch: SealifeLoader,
    pub facebook: FacebookOauth,
    pub frontend_cache: HybridCache<String, crate::frontend::CacheEntry>,
    pub frontend_url: String,
    pub admin_email: Option<String>,
    pub client: Client,
    pub openid_client: Option<OpenIDClient>,
}

impl SchemaContext {
    // This limit check should be placed wherever things can be brute forced, i.e, passwords
    fn check_limit(&self) -> Result<()> {
        match self.web.rate_limiter.check_key(&self.con.remote_ip) {
            Ok(_) => Ok(()),
            Err(err) => {
                let wait_time = err.wait_time_from(DefaultClock::default().now()).as_secs();

                Err(anyhow!("Too many requests! Try again in {} seconds", wait_time).into())
            }
        }
    }
}

pub fn get_limiter() -> IpLimiter {
    RateLimiter::keyed(
        Quota::with_period(Duration::from_secs(10))
            .unwrap()
            .allow_burst(NonZeroU32::new(3).unwrap()),
    )
}

pub struct Query;

#[Object]
impl Query {
    async fn site_url(&self, _context: &Context<'_>) -> FieldResult<String> {
        Ok(SITE_URL.clone())
    }

    async fn fb_app_id(&self, context: &Context<'_>) -> FieldResult<String> {
        let schema_context = context.data::<SchemaContext>()?;

        Ok(schema_context.web.facebook.fb_app_id.clone())
    }

    async fn openid_issuer_name(&self, context: &Context<'_>) -> FieldResult<Option<String>> {
        let schema_context = context.data::<SchemaContext>()?;

        Ok(schema_context
            .web
            .openid_client
            .as_ref()
            .map(|client| client.issuer_name()))
    }

    async fn categories(&self, context: &Context<'_>) -> FieldResult<Vec<Category>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .categories()
            .await?)
    }

    async fn category_values(&self, context: &Context<'_>) -> FieldResult<Vec<CategoryValue>> {
        Ok(context
            .data::<SchemaContext>()?
            .web
            .handle
            .category_values(None)
            .await?)
    }

    async fn search(
        &self,
        context: &Context<'_>,
        query: String,
        offset: Option<usize>,
    ) -> FieldResult<Vec<SearchResult>> {
        let schema_context = context.data::<SchemaContext>()?;

        let results = schema_context.web.searcher.search(&query, offset)?;

        Ok(results)
    }

    #[allow(clippy::too_many_arguments)]
    async fn dives(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        dive_site: Option<Uuid>,
        max_depth: Option<f64>,
        user_id: Option<Uuid>,
        username: Option<String>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<Dive>> {
        let schema_context = context.data::<SchemaContext>()?;

        let logged_in_id = schema_context.con.user.as_ref().map(|val| val.id);

        let query = DiveQuery {
            id,
            max_depth,
            dive_site,
            user_id,
            username,
            offset,
            limit: None,
        };

        let include_sites = context.look_ahead().field("diveSites").exists();

        let dives = schema_context
            .web
            .handle
            .dives(logged_in_id, &query)
            .await?;

        if include_sites {
            let sites_to_load: Vec<Uuid> =
                dives.iter().filter_map(|dive| dive.dive_site_id).collect();

            schema_context.web.dive_batch.load_many(sites_to_load).await;
        }

        Ok(dives)
    }

    async fn recent_dives(&self, context: &Context<'_>) -> FieldResult<Vec<Dive>> {
        let context = context.data::<SchemaContext>()?;

        Ok(context.web.handle.recent_dives().await?)
    }

    async fn user(&self, ctx: &Context<'_>, username: String) -> FieldResult<PublicUserInfo> {
        let context = ctx.data::<SchemaContext>()?;

        Ok(context.web.handle.user_by_username(&username).await?.into())
    }

    async fn current_user(&self, ctx: &Context<'_>) -> FieldResult<Option<LoginResponse>> {
        let context = ctx.data::<SchemaContext>()?;

        let user = match context.con.user {
            Some(ref user) => user.clone(),
            None => {
                return Ok(None);
            }
        };

        let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

        Ok(Some(LoginResponse {
            id: user.id,
            email: user.email.ok_or_else(|| anyhow!("No Email"))?,
            token,
            level: user.level,
            username: user.username,
            display_name: user.display_name,
            watermark_location: user.watermark_location,
            copyright_location: user.copyright_location,
            description: user.description,
            photo_id: user.photo_id,
            email_verified: user.email_verified,
        }))
    }

    async fn dive_sites(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        name: Option<String>,
        max_depth: Option<f64>,
        slug: Option<String>,
    ) -> FieldResult<Vec<DiveSite>> {
        let context = context.data::<SchemaContext>()?;

        let query = DiveSiteQuery {
            id,
            name,
            slug,
            max_depth,
        };

        Ok(context
            .web
            .handle
            .dive_sites(context.con.user.as_ref().map(|val| val.id), &query)
            .await?)
    }

    async fn popular_dive_sites(&self, context: &Context<'_>) -> FieldResult<Vec<DiveSite>> {
        let context = context.data::<SchemaContext>()?;

        Ok(context.web.handle.popular_dive_sites().await?)
    }

    #[allow(clippy::too_many_arguments)]
    async fn photos(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        user_id: Option<Uuid>,
        username: Option<String>,
        dive_site: Option<Uuid>,
        dive: Option<Uuid>,
        sealife_id: Option<Uuid>,
        offset: Option<usize>,
        order_by_upload: Option<bool>,
    ) -> FieldResult<Vec<Photo>> {
        let context = context.data::<SchemaContext>()?;

        let query = PhotoQuery {
            id,
            user_id,
            username,
            dive_site,
            dive,
            sealife_id,
            offset,
            limit: Some(10),
            order_by_upload,
        };

        Ok(context
            .web
            .handle
            .photos(context.con.user.as_ref(), &query)
            .await?)
    }

    #[allow(clippy::too_many_arguments)]
    async fn regions(&self, context: &Context<'_>) -> FieldResult<Vec<Region>> {
        let context = context.data::<SchemaContext>()?;

        Ok(context.web.handle.regions().await?)
    }

    async fn sealife(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        name: Option<String>,
        scientific_name: Option<String>,
        slug: Option<String>,
        category_values: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Sealife>> {
        let context = context.data::<SchemaContext>()?;

        let query = SealifeQuery {
            id,
            name,
            scientific_name,
            slug,
            photo_id: None,
            category_values,
        };

        Ok(context.web.handle.sealife(&query).await?)
    }

    async fn feedback(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
    ) -> FieldResult<Vec<Feedback>> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_admin() {
            Ok(context.web.handle.get_feedback(id).await?)
        } else {
            Err(anyhow!("Admin user level required").into())
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn request_reset_token(&self, context: &Context<'_>, email: String) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .web
            .handle
            .user(&email)
            .await
            .map_err(|_| anyhow!("Invalid Email Address"))?;

        let reset = context.web.handle.request_reset(user.id).await?;
        debug!("Reset token for {email} is: {}", reset.id);

        context
            .web
            .emailer
            .password_reset(email, reset)
            .await
            .map_err(|err| {
                error!("Error sending email: {}", err);

                anyhow!("Error sending reset email")
            })?;

        Ok(true)
    }

    async fn change_password(
        &self,
        context: &Context<'_>,
        old_password: String,
        new_password: String,
    ) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let email = user
            .email
            .as_ref()
            .ok_or_else(|| anyhow!("Email Required"))?;

        let existing_password = user
            .password
            .as_deref()
            .ok_or_else(|| anyhow!("Invalid Existing Password"))?;

        scrypt_check(&old_password, existing_password)
            .map_err(|_| anyhow!("Invalid Existing Password"))?;

        let params = ScryptParams::recommended();

        let hash = scrypt_simple(&new_password, &params)?;

        context.web.handle.update_password(email, &hash).await?;

        Ok(true)
    }

    async fn reset_password(
        &self,
        context: &Context<'_>,
        email: String,
        new_password: String,
        token: Uuid,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;
        let user = context.web.handle.user(&email).await?;

        let resets = context.web.handle.get_valid_resets(user.id).await?;

        if let Some(reset) = resets.iter().find(|reset| reset.id == token) {
            debug!("Valid password reset found:{reset:?}, resetting");

            let params = ScryptParams::recommended();

            let hash = scrypt_simple(&new_password, &params)?;

            context.web.handle.update_password(&email, &hash).await?;
            context.web.handle.remove_reset(reset.id).await?;

            let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

            Ok(LoginResponse {
                id: user.id,
                email,
                token,
                level: user.level,
                username: user.username,
                display_name: user.display_name,
                watermark_location: user.watermark_location,
                copyright_location: user.copyright_location,
                description: user.description,
                photo_id: user.photo_id,
                email_verified: user.email_verified,
            })
        } else {
            Err(anyhow!("This Password Reset Token is invalid or is expired.  Please try resetting your password again").into())
        }
    }

    async fn register_user(
        &self,
        context: &Context<'_>,
        username: String,
        email: String,
        password: String,
    ) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        if let Ok(_user) = context.web.handle.user(&email).await {
            return Err(anyhow!("Email address is already in use.  Try logging in instead").into());
        }

        let params = ScryptParams::recommended();

        let hash = scrypt_simple(&password, &params)?;

        let user = context
            .web
            .handle
            .new_user(&username, &email, Some(&hash))
            .await?;

        if let Some(ref admin_email) = context.web.admin_email {
            if admin_email.to_lowercase() == email.to_lowercase() {
                context
                    .web
                    .handle
                    .set_user_level(&email, UserLevel::Admin)
                    .await?;
            }
        }

        let verification = context
            .web
            .handle
            .request_email_verification(user.id)
            .await?;

        context
            .web
            .emailer
            .email_verification(email.clone(), verification)
            .await
            .context("Failed to send email verification")?;

        Ok(true)
    }

    async fn fb_register_user(
        &self,
        context: &Context<'_>,
        username: String,
        redirect_uri: String,
        code: String,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;

        let email = context
            .web
            .facebook
            .email_from_code(redirect_uri, code)
            .await?;

        if let Ok(_user) = context.web.handle.user(&email).await {
            return Err(anyhow!("Email address is already in use.  Try logging in instead").into());
        }

        let user = context.web.handle.new_user(&username, &email, None).await?;

        context.web.handle.mark_email_verified(user.id).await?;

        let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

        Ok(LoginResponse {
            id: user.id,
            email,
            token,
            level: UserLevel::User,
            username: user.username,
            display_name: user.display_name,
            watermark_location: user.watermark_location,
            copyright_location: user.copyright_location,
            description: user.description,
            photo_id: user.photo_id,
            email_verified: true,
        })
    }

    async fn login(
        &self,
        context: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;

        context.check_limit()?;

        let user =
            context.web.handle.user(&email).await.map_err(|_| {
                anyhow!("Unknown Email Address, Please make sure you've registered")
            })?;

        let existing_password = user
            .password
            .as_deref()
            .ok_or_else(|| anyhow!("Invalid Password"))?;

        scrypt_check(&password, existing_password).map_err(|_| anyhow!("Invalid Password"))?;

        let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

        Ok(LoginResponse {
            id: user.id,
            email,
            token,
            level: user.level,
            username: user.username,
            display_name: user.display_name,
            watermark_location: user.watermark_location,
            copyright_location: user.copyright_location,
            description: user.description,
            photo_id: user.photo_id,
            email_verified: user.email_verified,
        })
    }

    async fn fb_login(
        &self,
        context: &Context<'_>,
        redirect_uri: String,
        code: String,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;

        let email = context
            .web
            .facebook
            .email_from_code(redirect_uri, code)
            .await?;

        let user = context
            .web
            .handle
            .user(&email)
            .await
            .map_err(|_| anyhow!("Unknown Email Address, Please try Registering"))?;

        let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

        Ok(LoginResponse {
            id: user.id,
            email,
            token,
            level: user.level,
            username: user.username,
            display_name: user.display_name,
            watermark_location: user.watermark_location,
            copyright_location: user.copyright_location,
            description: user.description,
            photo_id: user.photo_id,
            email_verified: user.email_verified,
        })
    }

    async fn oauth_authorization_url(&self, context: &Context<'_>) -> FieldResult<String> {
        let context = context.data::<SchemaContext>()?;

        if let Some(ref openid_client) = context.web.openid_client {
            let url = openid_client.request_authorization_url().await?;

            Ok(url.to_string())
        } else {
            Err(anyhow!("OpenID not configured").into())
        }
    }

    async fn oauth_callback(
        &self,
        context: &Context<'_>,
        code: String,
        state: String,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;

        if let Some(ref openid_client) = context.web.openid_client {
            let values = openid_client.validate_callback(&code, &state).await?;

            let user = match context.web.handle.user(&values.email).await {
                Ok(user) => user,
                Err(_) => {
                    let user = context
                        .web
                        .handle
                        .new_user(&values.preferred_username, &values.email, None)
                        .await?;
                    context.web.handle.mark_email_verified(user.id).await?;

                    user
                }
            };

            let email = values.email;
            let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

            Ok(LoginResponse {
                id: user.id,
                email,
                token,
                level: user.level,
                username: user.username,
                display_name: user.display_name,
                watermark_location: user.watermark_location,
                copyright_location: user.copyright_location,
                description: user.description,
                photo_id: user.photo_id,
                email_verified: user.email_verified,
            })
        } else {
            Err(anyhow!("OpenID not configured").into())
        }
    }

    async fn update_settings(
        &self,
        context: &Context<'_>,
        display_name: Option<String>,
        watermark_location: OverlayLocation,
        copyright_location: Option<OverlayLocation>,
        description: String,
        photo_id: Option<Uuid>,
    ) -> FieldResult<Option<LoginResponse>> {
        let context = context.data::<SchemaContext>()?;

        let user = match context.con.user {
            Some(ref user) => user.clone(),
            None => {
                return Ok(None);
            }
        };

        let email = user.email.ok_or_else(|| anyhow!("No email"))?;

        let user = context
            .web
            .handle
            .update_settings(
                &email,
                display_name,
                watermark_location,
                copyright_location,
                description,
                photo_id,
            )
            .await?;

        let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

        Ok(Some(LoginResponse {
            id: user.id,
            email,
            token,
            level: user.level,
            username: user.username,
            display_name: user.display_name,
            watermark_location: user.watermark_location,
            copyright_location: user.copyright_location,
            description: user.description,
            photo_id: user.photo_id,
            email_verified: user.email_verified,
        }))
    }

    async fn sync_subsurface(
        &self,
        context: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user_id = context
            .con
            .user
            .as_ref()
            .map(|val| val.id)
            .ok_or_else(|| anyhow!("Login Required"))?;

        let mut repo = tokio::task::spawn_blocking(move || {
            subsurface::sync_repository(user_id, &email, &password)
        })
        .await??;

        subsurface::resolve_existing_sites(user_id, &mut repo, context.web.handle.clone()).await?;

        subsurface::import_repository(user_id, repo, context.web.handle.clone()).await?;

        Ok(true)
    }

    async fn add_feedback(&self, context: &Context<'_>, feedback: String) -> FieldResult<Feedback> {
        let context = context.data::<SchemaContext>()?;
        let user_id = context
            .con
            .user
            .as_ref()
            .map(|val| val.id)
            .ok_or_else(|| anyhow!("Login Required"))?;

        Ok(context.web.handle.add_feedback(user_id, feedback).await?)
    }

    async fn new_category(
        &self,
        context: &Context<'_>,
        category: CreateCategory,
    ) -> FieldResult<Category> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            Ok(context.web.handle.create_category(category).await?)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn remove_category(&self, context: &Context<'_>, category_id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            context.web.handle.remove_category(category_id).await?;
            Ok(true)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn remove_category_value(
        &self,
        context: &Context<'_>,
        category_value_id: Uuid,
    ) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            context
                .web
                .handle
                .remove_category_value(category_value_id)
                .await?;
            Ok(true)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn new_category_value(
        &self,
        context: &Context<'_>,
        category_value: CreateCategoryValue,
    ) -> FieldResult<CategoryValue> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            Ok(context
                .web
                .handle
                .create_category_value(category_value)
                .await?)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn new_dive(&self, context: &Context<'_>, dive: CreateDive) -> FieldResult<Dive> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if dive.published && !user.email_verified {
            return Err(anyhow!("Email Verification required before Publishing dives").into());
        }

        let was_published = if let Some(existing_id) = dive.id {
            context
                .web
                .handle
                .dives(
                    Some(user.id),
                    &DiveQuery {
                        id: Some(existing_id),
                        ..Default::default()
                    },
                )
                .await
                .ok()
                .and_then(|mut dives| dives.pop())
                .map(|dive| dive.published)
                .unwrap_or_default()
        } else {
            false
        };

        let dive = context.web.handle.create_dive(user.id, &dive, None).await?;
        context.web.handle.refresh_dives(user.id).await?;

        let followers = context
            .web
            .handle
            .followers(user.id)
            .await?
            .into_iter()
            .map(|user| user.ap_inbox())
            .collect::<Vec<_>>();

        CreatePost::send(dive.clone(), followers, &context.fed, was_published).await?;

        Ok(dive)
    }

    async fn remove_dive(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.remove_dive(user.id, id).await?;

        Ok(true)
    }

    async fn like_dive(&self, context: &Context<'_>, dive_id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.like_dive(user.id, dive_id).await?;

        Ok(true)
    }

    async fn unlike_dive(&self, context: &Context<'_>, dive_id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.unlike_dive(user.id, dive_id).await?;

        Ok(true)
    }

    async fn new_comment(
        &self,
        context: &Context<'_>,
        comment: CreateDiveComment,
    ) -> FieldResult<DiveComment> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        Ok(context.web.handle.create_comment(user.id, &comment).await?)
    }

    async fn remove_comment(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.remove_comment(id, user.id).await?;

        Ok(true)
    }

    async fn new_dive_site(
        &self,
        context: &Context<'_>,
        site: CreateDiveSite,
    ) -> FieldResult<DiveSite> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if site.published && !user.email_verified {
            return Err(anyhow!(
                "Email Verification required before Editing/Publishing dive sites"
            )
            .into());
        }

        let dive_site = context.web.handle.create_dive_site(user.id, &site).await?;

        context
            .web
            .searcher
            .build_index(&context.web.handle)
            .await?;

        Ok(dive_site)
    }

    async fn merge_dive_sites(
        &self,
        context: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let dive_site = context
            .web
            .handle
            .dive_sites(
                Some(user.id),
                &DiveSiteQuery {
                    id: Some(from_id),
                    name: None,
                    max_depth: None,
                    slug: None,
                },
            )
            .await?
            .pop()
            .ok_or_else(|| anyhow!("Dive site not found"))?;

        // Check it exists
        context
            .web
            .handle
            .dive_sites(
                Some(user.id),
                &DiveSiteQuery {
                    id: Some(from_id),
                    name: None,
                    max_depth: None,
                    slug: None,
                },
            )
            .await?
            .pop()
            .ok_or_else(|| anyhow!("Dive site not found"))?;

        if user.is_editor() || dive_site.user_id == Some(user.id) {
            context.web.handle.merge_dive_sites(from_id, to_id).await?;
            context
                .web
                .searcher
                .build_index(&context.web.handle)
                .await?;

            return Ok(true);
        }

        Ok(false)
    }

    async fn remove_dive_site(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let dive_site = context
            .web
            .handle
            .dive_sites(
                Some(user.id),
                &DiveSiteQuery {
                    id: Some(id),
                    name: None,
                    max_depth: None,
                    slug: None,
                },
            )
            .await?
            .pop()
            .ok_or_else(|| anyhow!("Dive site not found"))?;

        if user.is_editor() || dive_site.user_id == Some(user.id) {
            context.web.handle.remove_dive_site(id).await?;
            context
                .web
                .searcher
                .build_index(&context.web.handle)
                .await?;

            return Ok(true);
        }

        Ok(false)
    }

    async fn delete_user(&self, context: &Context<'_>, password: String) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let existing_password = user
            .password
            .as_deref()
            .ok_or_else(|| anyhow!("Invalid Password"))?;
        scrypt_check(&password, existing_password).map_err(|_| anyhow!("Invalid Password"))?;

        context.web.handle.delete_user(user.id).await?;

        Ok(true)
    }

    async fn update_photo(
        &self,
        context: &Context<'_>,
        mut photo: CreatePhoto,
    ) -> FieldResult<Photo> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let photo_id = photo.id.ok_or_else(|| anyhow!("No Photo ID"))?;

        let existing_photo = context
            .web
            .handle
            .photos(Some(user), &PhotoQuery::id(photo_id))
            .await?
            .pop()
            .ok_or_else(|| anyhow!("Photo not found"))?;

        photo.size = existing_photo.size;
        photo.dive_id = existing_photo.dive_id;

        if user.is_editor() || photo.user_id == user.id {
            let new_photo = context.web.handle.add_photo(&photo).await?;
            return Ok(new_photo);
        }

        Err(anyhow!("No permissions to edit this photo").into())
    }

    async fn remove_photo(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let photo = context
            .web
            .handle
            .photos(Some(user), &PhotoQuery::id(id))
            .await?
            .pop()
            .ok_or_else(|| anyhow!("Photo not found"))?;

        if user.is_editor() || photo.user_id == user.id {
            context.web.handle.remove_photo(id).await?;

            return Ok(true);
        }

        Ok(false)
    }

    async fn like_photo(&self, context: &Context<'_>, photo_id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.like_photo(user.id, photo_id).await?;

        Ok(true)
    }

    async fn unlike_photo(&self, context: &Context<'_>, photo_id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        context.web.handle.unlike_photo(user.id, photo_id).await?;

        Ok(true)
    }

    async fn new_sealife(
        &self,
        context: &Context<'_>,
        mut sealife: CreateSealife,
    ) -> FieldResult<Sealife> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        // Only allow editors and above the ability to edit whether location is hidden
        if !user.is_editor() {
            if let Some(id) = sealife.id {
                sealife.hide_location = context
                    .web
                    .handle
                    .sealife(&SealifeQuery::id(id))
                    .await?
                    .pop()
                    .map(|val| val.hide_location)
                    .unwrap_or_default();
            }
        }

        let sealife = context.web.handle.create_sealife(&sealife).await?;

        context
            .web
            .searcher
            .build_index(&context.web.handle)
            .await?;

        Ok(sealife)
    }

    async fn remove_sealife(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;

        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            context.web.handle.remove_sealife(id).await?;
            context
                .web
                .searcher
                .build_index(&context.web.handle)
                .await?;
        }

        Ok(true)
    }

    async fn check_reference(
        &self,
        context: &Context<'_>,
        url: String,
    ) -> FieldResult<OgReference> {
        let context = context.data::<SchemaContext>()?;

        let reference = OgReference::from_url(&context.web.handle.client, &url).await?;

        Ok(reference)
    }

    async fn new_reference(
        &self,
        context: &Context<'_>,

        url: String,
        sealife_id: Option<Uuid>,
        dive_site_id: Option<Uuid>,
    ) -> FieldResult<OgReference> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        let create_reference = CreateOgReference {
            url,
            sealife_id,
            dive_site_id,
        };

        if user.is_editor() {
            Ok(context.web.handle.add_reference(create_reference).await?)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn remove_reference(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_editor() {
            context.web.handle.remove_reference(id).await?;
            Ok(true)
        } else {
            Err(anyhow!("Editor user level required").into())
        }
    }

    async fn new_region(&self, context: &Context<'_>, region: CreateRegion) -> FieldResult<Region> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_admin() {
            Ok(context.web.handle.add_region(region).await?)
        } else {
            Err(anyhow!("Admin user level required").into())
        }
    }

    async fn remove_region(&self, context: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        let context = context.data::<SchemaContext>()?;
        let user = context
            .con
            .user
            .as_ref()
            .ok_or_else(|| anyhow!("Login Required"))?;

        if user.is_admin() {
            context.web.handle.remove_region(id).await?;
            Ok(true)
        } else {
            Err(anyhow!("Admin user level required").into())
        }
    }

    async fn verify_email(
        &self,
        context: &Context<'_>,
        email: String,
        token: Uuid,
    ) -> FieldResult<LoginResponse> {
        let context = context.data::<SchemaContext>()?;

        let user = context.web.handle.user(&email).await?;
        let verifications = context
            .web
            .handle
            .get_valid_email_verifications(user.id)
            .await?;

        if let Some(verification) = verifications.iter().find(|reset| reset.id == token) {
            debug!("Valid verification found:{verification:?}");

            context.web.handle.mark_email_verified(user.id).await?;
            let token = context.web.cipher.base64_encrypt(user.id.as_bytes())?;

            Ok(LoginResponse {
                id: user.id,
                email,
                token,
                level: user.level,
                username: user.username,
                display_name: user.display_name,
                watermark_location: user.watermark_location,
                copyright_location: user.copyright_location,
                description: user.description,
                photo_id: user.photo_id,
                email_verified: true,
            })
        } else {
            Err(anyhow!("This Email Verification Token is invalid or is expired.").into())
        }
    }

    async fn resend_verification(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let context = ctx.data::<SchemaContext>()?;

        let user = match context.con.user {
            Some(ref user) => user,
            None => {
                return Err(anyhow!("You must be logged in to resend a verification email").into())
            }
        };

        let email = user
            .email
            .as_ref()
            .ok_or_else(|| anyhow!("No email associated with user"))?;

        // Check if already verified
        if user.email_verified {
            return Err(anyhow!("Email is already verified").into());
        }

        // Create new verification token and send email
        let verification = context
            .web
            .handle
            .request_email_verification(user.id)
            .await?;
        context
            .web
            .emailer
            .email_verification(email.to_string(), verification)
            .await?;

        Ok(true)
    }

    async fn plan_dive(&self, plan: DivePlanInput) -> FieldResult<DiveSchedule> {
        let schedule = crate::plan::plan_dive(plan)?;

        Ok(schedule)
    }
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn schema() -> Schema {
    Schema::build(Query, Mutation, EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .finish()
}
