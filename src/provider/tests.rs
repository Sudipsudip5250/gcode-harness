use super::*;
use crate::provider::models::{ensure_model_allowed_for_subscription, filtered_display_models};

fn with_clean_provider_test_env<T>(f: impl FnOnce() -> T) -> T {
    let _guard = crate::storage::lock_test_env();
    let temp = tempfile::tempdir().expect("tempdir");
    let prev_home = std::env::var_os("GCODE_HOME");
    let prev_subscription =
        std::env::var_os(crate::subscription_catalog::GCODE_SUBSCRIPTION_ACTIVE_ENV);
    let saved_profile_env = [
        "OPENROUTER_API_KEY",
        "DEEPSEEK_API_KEY",
        "KIMI_API_KEY",
        "GCODE_OPENROUTER_API_BASE",
        "GCODE_OPENROUTER_API_KEY_NAME",
        "GCODE_OPENROUTER_ENV_FILE",
        "GCODE_OPENROUTER_CACHE_NAMESPACE",
        "GCODE_OPENROUTER_PROVIDER_FEATURES",
        "GCODE_OPENROUTER_ALLOW_NO_AUTH",
        "GCODE_OPENROUTER_MODEL_CATALOG",
        "GCODE_OPENROUTER_MODEL",
        "GCODE_OPENROUTER_STATIC_MODELS",
        "GCODE_OPENAI_COMPAT_API_BASE",
        "GCODE_OPENAI_COMPAT_API_KEY_NAME",
        "GCODE_OPENAI_COMPAT_DEFAULT_MODEL",
        "OPENAI_API_KEY",
        "GCODE_NAMED_PROVIDER_PROFILE",
        "GCODE_PROVIDER_PROFILE_ACTIVE",
        "GCODE_PROVIDER_PROFILE_NAME",
    ]
    .map(|key| (key, std::env::var_os(key)));
    crate::env::set_var("GCODE_HOME", temp.path());
    for (key, _) in &saved_profile_env {
        crate::env::remove_var(key);
    }
    crate::subscription_catalog::clear_runtime_env();
    crate::auth::claude::set_active_account_override(None);
    crate::auth::codex::set_active_account_override(None);

    let result = f();

    crate::auth::claude::set_active_account_override(None);
    crate::auth::codex::set_active_account_override(None);
    if let Some(prev_home) = prev_home {
        crate::env::set_var("GCODE_HOME", prev_home);
    } else {
        crate::env::remove_var("GCODE_HOME");
    }
    if let Some(prev_subscription) = prev_subscription {
        crate::env::set_var(
            crate::subscription_catalog::GCODE_SUBSCRIPTION_ACTIVE_ENV,
            prev_subscription,
        );
    } else {
        crate::env::remove_var(crate::subscription_catalog::GCODE_SUBSCRIPTION_ACTIVE_ENV);
    }
    for (key, value) in saved_profile_env {
        if let Some(value) = value {
            crate::env::set_var(key, value);
        } else {
            crate::env::remove_var(key);
        }
    }
    crate::subscription_catalog::clear_runtime_env();
    result
}

fn enter_test_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime")
}

fn with_env_var<T>(key: &str, value: &str, f: impl FnOnce() -> T) -> T {
    let prev = std::env::var_os(key);
    crate::env::set_var(key, value);
    let result = f();
    if let Some(prev) = prev {
        crate::env::set_var(key, prev);
    } else {
        crate::env::remove_var(key);
    }
    result
}

fn test_multi_provider_with_cursor() -> MultiProvider {
    MultiProvider {
        claude: RwLock::new(None),
        anthropic: RwLock::new(None),
        openai: RwLock::new(None),
        copilot_api: RwLock::new(None),
        antigravity: RwLock::new(None),
        gemini: RwLock::new(None),
        cursor: RwLock::new(Some(Arc::new(cursor::CursorCliProvider::new()))),
        openrouter: RwLock::new(None),
        active: RwLock::new(ActiveProvider::Cursor),
        use_claude_cli: false,
        startup_notices: RwLock::new(Vec::new()),
        forced_provider: None,
    }
}

include!("tests/auth_refresh.rs");
include!("tests/model_resolution.rs");
include!("tests/fallback_failover.rs");
include!("tests/catalog_subscription.rs");
