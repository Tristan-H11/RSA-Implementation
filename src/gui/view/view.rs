use druid::{Command, Lens, Selector, Widget, WidgetExt, widget::{Button, Flex, Label, TextBox, ViewSwitcher}, Env};
use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel, AppState, View};
use crate::gui::gui::{SWITCH_TO_ALICE, SWITCH_TO_BOB, SWITCH_TO_HAUPTMENU, UPDATE_PUBLIC_KEY};


pub(crate) fn build_haupt_menu() -> impl Widget<HauptMenuModel> {
    // Entry-Felder
    let p1_entry = Flex::row()
        .with_child(Label::new("Eingabe P1"))
        .with_child(TextBox::new().lens(HauptMenuModel::eingabe_p1));

    let p2_entry = Flex::row()
        .with_child(Label::new("Eingabe P2"))
        .with_child(TextBox::new().lens(HauptMenuModel::eingabe_p2));

    let miller_rabin_entry = Flex::row()
        .with_child(Label::new("Eingabe Miller-Rabin"))
        .with_child(TextBox::new().lens(HauptMenuModel::eingabe_miller_rabin));

    // Button
    let calc_open_key_button = Button::new("Berechne Öffentlichen Schlüssel").on_click(|ctx, data: &mut HauptMenuModel, _env| {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        // Angenommen, die berechneten öffentlichen Schlüssel sind "1234567890" für Alice und "0987654321" für Bob
        data.open_key_alice = "1234567890".to_string();
        data.open_key_bob = "0987654321".to_string();
    });

    let open_alice_button = Button::new("Öffne Alice Ansicht").on_click(|_ctx, _data, _env| {
        _ctx.submit_command(SWITCH_TO_ALICE);
    });
    let open_bob_button = Button::new("Öffne Bob Ansicht").on_click(|_ctx, _data, _env| {
        _ctx.submit_command(SWITCH_TO_BOB);
    });

    // Label
    let open_key_alice_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Alice: {}", &data.open_key_alice)
    });

    let open_key_bob_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Bob: {}", &data.open_key_bob)
    });

    Flex::column()
        .with_child(p1_entry)
        .with_default_spacer()
        .with_child(p2_entry)
        .with_default_spacer()
        .with_child(miller_rabin_entry)
        .with_default_spacer()
        .with_child(calc_open_key_button)
        .with_default_spacer()
        .with_child(open_key_alice_label)
        .with_default_spacer()
        .with_child(open_key_bob_label)
        .with_default_spacer()
        .with_child(open_alice_button)
        .with_default_spacer()
        .with_child(open_bob_button)
        .padding(druid::Insets::uniform(10.0))
}

pub(crate) fn build_alice_view() -> impl Widget<AliceModel> {
    // Label
    let secret_key_label = Label::new(|data: &AliceModel, _env: &Env| {
        format!("Geheimschlüssel: {}", data.anzeige_geheimer_schluessel)
    });

    // Entry-Felder und Labels
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Klartext: "))
        .with_child(TextBox::new().lens(AliceModel::eingabe_klartext));

    let signature_row = Flex::row()
        .with_child(
            Flex::column()
                .with_child(TextBox::new().with_placeholder("Signatur").lens(AliceModel::anzeige_signatur))
        )
        .with_default_spacer()
        .with_child(
            Label::new(|data: &AliceModel, _env: &Env| {
                if data.status_signatur {
                    "Gültig".to_string()
                } else {
                    "Ungültig".to_string()
                }
            })
        );

    // Buttons
    let encrypt_button = Button::new("Verschlüsseln").on_click(|_ctx, data: &mut AliceModel, _env| {
        // todo -- Logik für Verschlüsselung
    });
    let sign_button = Button::new("Signieren").on_click(|_ctx, data: &mut AliceModel, _env| {
        // todo -- Logik für Signierung
    });
    let decrypt_button = Button::new("Entschlüsseln").on_click(|_ctx, data: &mut AliceModel, _env| {
        // todo -- Logik für Entschlüsselung
    });
    let send_message_button = Button::new("Nachricht senden").on_click(|_ctx, data: &mut AliceModel, _env| {
        // todo -- Logik zum Senden der Nachricht
    });
    let clear_button = Button::new("Clear").on_click(|_ctx, data: &mut AliceModel, _env| {
        data.eingabe_klartext.clear();
        data.anzeige_signatur.clear();
        data.status_signatur = false;
        // usw.
    });
    let back_button = Button::new("Zurück zum Hauptmenü").on_click(|_ctx, _data: &mut AliceModel, _env| {
        _ctx.submit_command(SWITCH_TO_HAUPTMENU);
    });

    Flex::column()
        .with_child(secret_key_label)
        .with_default_spacer()
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(encrypt_button)
        .with_default_spacer()
        .with_child(sign_button)
        .with_default_spacer()
        .with_child(decrypt_button)
        .with_default_spacer()
        .with_child(send_message_button)
        .with_default_spacer()
        .with_child(signature_row)
        .with_default_spacer()
        .with_child(clear_button)
        .with_default_spacer()
        .with_child(back_button)
        .padding(druid::Insets::uniform(10.0))
}

pub(crate) fn build_bob_view() -> impl Widget<BobModel> {
    // Label
    let secret_key_label = Label::new(|data: &BobModel, _env: &Env| {
        format!("Geheimschlüssel: {}", data.anzeige_geheimer_schluessel)
    });

    // Entry
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Klartext: "))
        .with_child(TextBox::new().lens(BobModel::eingabe_klartext));

    let signature_row = Flex::row()
        .with_child(
            Flex::column()
                .with_child(TextBox::new().with_placeholder("Signatur").lens(BobModel::anzeige_signatur))
        )
        .with_default_spacer()
        .with_child(
            Label::new(|data: &BobModel, _env: &Env| {
                if data.status_signatur {
                    "Gültig".to_string()
                } else {
                    "Ungültig".to_string()
                }
            })
        );

    // Buttons
    let encrypt_button = Button::new("Verschlüsseln").on_click(|_ctx, data: &mut BobModel, _env| {
        // todo -- Logik für Verschlüsselung
    });
    let sign_button = Button::new("Signieren").on_click(|_ctx, data: &mut BobModel, _env| {
        // todo -- Logik für Signierung
    });
    let decrypt_button = Button::new("Entschlüsseln").on_click(|_ctx, data: &mut BobModel, _env| {
        // todo -- Logik für Entschlüsselung
    });
    let send_message_button = Button::new("Nachricht senden").on_click(|_ctx, data: &mut BobModel, _env| {
        // todo -- Logik zum Senden der Nachricht
    });
    let clear_button = Button::new("Clear").on_click(|_ctx, data: &mut BobModel, _env| {
        data.eingabe_klartext.clear();
        data.anzeige_signatur.clear();
        data.status_signatur = false;
        // usw.
    });
    let back_button = Button::new("Zurück zum Hauptmenü").on_click(|_ctx, _data: &mut BobModel, _env| {
        _ctx.submit_command(SWITCH_TO_HAUPTMENU);
    });

    Flex::column()
        .with_child(secret_key_label)
        .with_default_spacer()
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(encrypt_button)
        .with_default_spacer()
        .with_child(sign_button)
        .with_default_spacer()
        .with_child(decrypt_button)
        .with_default_spacer()
        .with_child(send_message_button)
        .with_default_spacer()
        .with_child(signature_row)
        .with_default_spacer()
        .with_child(clear_button)
        .with_default_spacer()
        .with_child(back_button)
        .padding(druid::Insets::uniform(10.0))
}
