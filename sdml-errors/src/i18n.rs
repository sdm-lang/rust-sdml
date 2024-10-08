/*!
Internal
 */

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use rust_embed::RustEmbed;
use std::sync::OnceLock;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! i18n {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!(
            $crate::i18n::LANGUAGE_LOADER.get_or_init($crate::i18n::init_translations),
            $message_id
        )
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!(
            $crate::i18n::LANGUAGE_LOADER.get_or_init($crate::i18n::init_translations),
            $message_id,
            $( $args )*
        )
    }};
}

// ------------------------------------------------------------------------------------------------
// Language Translation
// ------------------------------------------------------------------------------------------------

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub(crate) static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub(crate) fn init_translations() -> FluentLanguageLoader {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .unwrap();
    loader
}
