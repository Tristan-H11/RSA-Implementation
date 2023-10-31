use druid::{
    Widget, widget::Flex, WidgetExt,
};
use druid::widget::SizedBox;

use crate::gui::controller::commands::{
    CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::BobModel;
use crate::gui::view::traits::common_view_builder::{CommonViewComponentsDefault, ViewBuilder};

pub struct BobViewBuilder;

impl ViewBuilder<BobModel> for BobViewBuilder {
    fn build_view() -> SizedBox<BobModel> {
        let common_components = CommonViewComponentsDefault::new();

        // Entry-Felder und Labels
        let plaintext_entry = common_components.create_text_entry_default(
            "Klartext: ",
            "z.B. Hallo wie geht es dir?",
            false,
            BobModel::plaintext,
        );

        let ciphertext_entry = common_components.create_text_entry_default(
            "Geheimtext: ",
            "Wird automatisch berechnet",
            false,
            BobModel::ciphertext,
        );

        let secret_exponent_entry = common_components.create_text_entry_default(
            "Geheimer Exponent: ",
            "Wird automatisch berechnet",
            true,
            BobModel::private_exponent,
        );

        let signature_row =
            common_components.create_text_entry_default("Signatur: ", "Signatur", true, BobModel::signature);

        // Buttons
        let encrypt_button =
            common_components.create_button_default("Mit Alice PublicKey verschlüsseln", ENCRYPT);
        let calc_sign_button = common_components.create_button_default("Signatur berechnen", SIGN);
        let check_sign_button = common_components.create_button_default("Signatur prüfen", VERIFY);
        let decrypt_button =
            common_components.create_button_default("Mit eigenem PrivateKey entschlüsseln", DECRYPT);
        let send_message_button = common_components.create_button_default("Nachricht senden", SEND_MESSAGE);
        let clear_button =
            common_components.create_button_default("Alles außer privaten Schlüssel löschen", CLEAR);
        let back_button =
            common_components.create_button_default("Zurück zum Hauptmenü", SWITCH_TO_MAIN_MENU);

        Flex::column()
            .with_flex_spacer(common_components.flex_space_default)
            .with_flex_child(plaintext_entry, 0.2)
            .with_flex_child(ciphertext_entry, 0.2)
            .with_flex_child(encrypt_button, 0.1)
            .with_flex_child(decrypt_button, 0.1)
            .with_flex_child(secret_exponent_entry, 0.2)
            .with_flex_child(calc_sign_button, 0.1)
            .with_flex_child(check_sign_button, 0.1)
            .with_flex_child(signature_row, 0.2)
            .with_flex_child(send_message_button, 0.1)
            .with_flex_child(clear_button, 0.1)
            .with_flex_child(back_button, 0.1)
            .expand_height()
    }
}
