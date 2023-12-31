use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::One;
use log::{debug, trace};
use num::Integer;

use crate::encryption::math_functions::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::rsa::keys::{RsaKey, RsaKeyType};
use crate::shared::errors::RsaError::KeyGenerationError;
use crate::shared::errors::{ArithmeticError, RsaError};

///
/// Ein Service zum Generieren von Schlüsselpaaren für RSA.
///
pub struct RsaKeygenService {
    key_size: u32,
    number_theory_service: NumberTheoryService,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    /// * `key_size` - Die Breite des Moduls `n`, mit welchem die Schlüssel berechnet werden.
    ///
    pub fn new(key_size: u32, number_theory_service: NumberTheoryService) -> RsaKeygenService {
        debug!(
            "Erstellen eines neuen RsaKeygenService mit key_size {}",
            key_size
        );
        RsaKeygenService {
            key_size,
            number_theory_service,
        }
    }

    ///
    /// Generiert ein Schlüsselpaar für RSA.
    ///
    /// # Argumente
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_seed` - Der Seed für die gleichverteilte Zufallszahlerzeugung.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    ///
    /// # Fehler
    /// * `RsaError::KeyGenerationError` - Falls die Schlüsselerzeugung fehlschlägt.
    pub(crate) fn generate_keypair(
        &self,
        miller_rabin_iterations: u32,
        random_seed: u32,
    ) -> Result<(RsaKey, RsaKey), RsaError> {
        debug!(
            "Generiere Schlüsselpaar mit key_size {} und Miller-Rabin-Iterations {}",
            self.key_size, miller_rabin_iterations
        );
        let random_generator = &PseudoRandomNumberGenerator::new(random_seed);

        let (prime_one, prime_two) =
            self.get_distinct_primes(miller_rabin_iterations, random_generator);

        let n = &prime_one * &prime_two;
        debug!("n ist {}", n);

        let phi = (&prime_one - BigInt::one()) * (&prime_two - BigInt::one());
        let e = self.generate_e(&phi, random_generator);
        let d = self.generate_d(&e, &phi);

        match d {
            Ok(d) => {
                let public_key = RsaKey::new(RsaKeyType::Public, e, n.clone());
                let private_key = RsaKey::new(RsaKeyType::Private, d, n);
                debug!("Schlüsselpaar generiert");
                Ok((public_key, private_key))
            }
            Err(_) => Err(KeyGenerationError),
        }
    }

    /// Generiert zwei verschiedene Primzahlen mit der angegebenen Breite.
    ///
    /// # Argumente
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    fn get_distinct_primes(
        &self,
        miller_rabin_iterations: u32,
        random_generator: &PseudoRandomNumberGenerator,
    ) -> (BigInt, BigInt) {
        let (prim_size_one, prim_size_two) = if self.key_size.is_even() {
            (self.key_size / 2, self.key_size / 2)
        } else {
            (self.key_size / 2 + 1, self.key_size / 2)
        };
        let n_counter = RelaxedCounter::new(1);
        let prime_one = self.generate_prime(
            prim_size_one,
            miller_rabin_iterations,
            random_generator,
            &n_counter,
        );
        let mut prime_two = self.generate_prime(
            prim_size_two,
            miller_rabin_iterations,
            random_generator,
            &n_counter,
        );

        while prime_one == prime_two {
            trace!(
                "Generierter prime_one {} ist gleich prime_two {}. Starte neuen Versuch",
                prime_one,
                prime_two
            );
            prime_two = self.generate_prime(
                prim_size_two,
                miller_rabin_iterations,
                random_generator,
                &n_counter,
            );
        }
        (prime_one, prime_two)
    }

    /// Generiert eine Primzahl mit der angegebenen Breite.
    ///
    /// # Argumente
    /// * `size` - Die Breite der Primzahl.
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    /// * `n_counter` - Der Zähler für den Zugriff auf die Zufallsfolge. Achtung: Der Zähler wird inkrementiert!
    ///
    /// # Rückgabe
    /// Die generierte Primzahl.
    fn generate_prime(
        &self,
        size: u32,
        miller_rabin_iterations: u32,
        random_generator: &PseudoRandomNumberGenerator,
        n_counter: &RelaxedCounter,
    ) -> BigInt {
        debug!(
            "Generiere eine Primzahl mit size {} und Miller-Rabin-Iterations {}",
            size, miller_rabin_iterations
        );

        let upper_bound = &BigInt::from(2).pow(size);
        let lower_bound = &BigInt::from(2).pow(size - 1);

        let mut prime_candidate = random_generator.take_uneven(lower_bound, upper_bound, n_counter);

        while !self.number_theory_service.is_probably_prime(
            &prime_candidate,
            miller_rabin_iterations,
            random_generator,
        ) {
            trace!(
                "Generierter Primkandidat {} ist keine Primzahl",
                prime_candidate
            );
            prime_candidate = random_generator.take_uneven(lower_bound, upper_bound, n_counter);
        }
        debug!(
            "Generierter Primkandidat {} ist eine Primzahl",
            prime_candidate
        );
        prime_candidate
    }

    /// Generiert eine Zahl `e` mit `1 < e < phi` und `ggT(e, phi) = 1`.
    ///
    /// # Argumente
    /// * `phi` - Die Zahl `phi`.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `e`.
    fn generate_e(&self, phi: &BigInt, random_generator: &PseudoRandomNumberGenerator) -> BigInt {
        debug!("Generiere e mit phi {}", phi);

        let n_counter = RelaxedCounter::new(1);

        let mut e = random_generator.take(&3.into(), &phi.decrement(), &n_counter);
        while e < *phi {
            let ggt = self.number_theory_service.extended_euclid(&e, phi).ggt;
            if ggt.is_one() {
                debug!("Generierter e {} ist relativ prim zu phi {}", e, phi);
                return e;
            }
            trace!("Generierter e {} ist nicht relativ prim zu phi {}", e, phi);
            e.increment_assign();
        }
        panic!("Kein e gefunden, das relativ prim zu phi {} ist", phi);
    }

    /// Generiert eine Zahl `d` mit `1 < d < phi` und `e * d = 1 mod phi`.
    /// d ist damit das multiplikative Inverse von e mod phi.
    ///
    /// # Argumente
    /// * `e` - Die Zahl `e`.
    /// * `phi` - Die Zahl `phi`.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `d`.
    ///
    /// # Fehler
    /// * `ArithmeticError::NoInverseError` - Falls kein multiplikatives Inverses gefunden werden konnte.
    fn generate_d(&self, e: &BigInt, phi: &BigInt) -> Result<BigInt, ArithmeticError> {
        trace!("Generiere d mit e {} und phi {}", e, phi);
        let d = self.number_theory_service.modulo_inverse(e, phi)?;
        debug!("d ist {}", d);
        Ok(d)
    }
}
