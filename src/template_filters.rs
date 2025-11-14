#![allow(clippy::box_default)]

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToUpperCamelCase,
};
use minijinja::Environment;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::template::LiquidObjectResource;

// This file is now mainly a placeholder since minijinja filters are registered
// directly in template.rs using add_filter. The case conversion functions below
// are used by those filters.

/// Helper to register all template filters with a minijinja environment
pub fn register_all_filters(
    env: &mut Environment,
    _template_dir: PathBuf,
    _liquid_object: LiquidObjectResource,
    _allow_commands: bool,
    _silent: bool,
    _rhai_filter_files: Arc<Mutex<Vec<PathBuf>>>,
) {
    // Register case conversion filters
    env.add_filter("kebab_case", |s: String| -> String { s.to_kebab_case() });
    env.add_filter("lower_camel_case", |s: String| -> String { s.to_lower_camel_case() });
    env.add_filter("pascal_case", |s: String| -> String { s.to_pascal_case() });
    env.add_filter("shouty_kebab_case", |s: String| -> String { s.to_shouty_kebab_case() });
    env.add_filter("shouty_snake_case", |s: String| -> String { s.to_shouty_snake_case() });
    env.add_filter("snake_case", |s: String| -> String { s.to_snake_case() });
    env.add_filter("title_case", |s: String| -> String { s.to_title_case() });
    env.add_filter("upper_camel_case", |s: String| -> String { s.to_upper_camel_case() });
}
