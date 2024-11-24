use leptos_i18n::*;
use nova_forms::*;
use std::num::ParseIntError;
use super::demo_form::RadioValue;
use crate::app::use_i18n;

pub fn provide_translations() {
    let i18n = use_i18n();

    // Define custom error message translations.
    provide_translation(move |err| match err {
        NonEmptyStringError::EmptyString => t!(i18n, error_empty_string).into(),
    });
    provide_translation(move |err| match err {
        EmailError::InvalidFormat => t!(i18n, error_invalid_format).into(),
    });
    provide_translation(move |err| match err {
        TelephoneError::InvalidFormat => t!(i18n, error_invalid_format).into(),
    });
    provide_translation(move |err| match err {
        AcceptError::NotAccepted => t!(i18n, error_not_accepted).into(),
    });
    provide_translation(move |_: ParseIntError| t!(i18n, error_invalid_format).into());
    provide_translation(move |err| match err {
        RadioValue::A => t!(i18n, radio_a).into(),
        RadioValue::B => t!(i18n, radio_b).into(),
        RadioValue::C => t!(i18n, radio_c).into(),
    });
    provide_translation(move |err| match err {
        SubmitState::Initial => "".into(),
        SubmitState::Error(_) => t!(i18n, submit_error).into(),
        SubmitState::Pending => t!(i18n, submit_pending).into(),
        SubmitState::Success => t!(i18n, submit_success).into(),
    });
    provide_translation(move |toolbar| match toolbar {
        Translation::Edit => t!(i18n, edit).into(),
        Translation::Submit => t!(i18n, submit).into(),
        Translation::Preview => t!(i18n, preview).into(),
        Translation::Language => t!(i18n, language).into(),
        Translation::Menu => t!(i18n, menu).into(),
    });
}