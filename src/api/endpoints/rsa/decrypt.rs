use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;

use crate::api::serializable_models::{EncryptDecryptRequest, SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryService;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

/// Endpunkt zum Entschlüsseln einer Nachricht mit RSA.
/// # Argumente
/// * `req_body` - Die Anfrage, die den verschlüsselten Text und den privaten Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die den entschlüsselten Text enthält.
pub(crate) async fn decrypt(
    req_body: Json<EncryptDecryptRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/decrypt wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: EncryptDecryptRequest = req_body.into_inner();
    let use_fast = query.use_fast;
    let ciphertext = req_body.message;
    let number_system_base = req_body.number_system_base;

    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.key_pair.to_private_key()?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let rsa_service =
            crate::encryption::rsa::rsa_service::RsaService::new(number_theory_service);

        let plaintext = rsa_service.decrypt(&ciphertext, number_system_base, &private_key);
        let response = SingleStringResponse { message: plaintext };

        Ok(HttpResponse::Ok().json(response))
    })
}
