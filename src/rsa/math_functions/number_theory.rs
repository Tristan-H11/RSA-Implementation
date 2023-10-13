use crate::rsa::math_functions::big_int_util::{is_even, is_one, is_zero};
use ibig::ops::RemEuclid;
use ibig::{ubig, UBig};
use mod_exp::mod_exp;
use rand::{thread_rng, Rng};
use std::ops::Div;

///
/// Schnelle Exponentiation der Potenz und Reduzierung um einen Modul.
/// Alternativer Ansatz von Herrn Elsner zur schnellen Exponentiation durch Halbieren der Potenz.
///
/// # Argumente
/// * `base` - Die Basis, von welcher die Potenz berechnet werden soll.
/// * `exponent`- Der Exponent zur Berechnung der Potenz.
/// * `modul` - Der Modul, durch den reduziert werden soll.
///
/// # Beispiel
/// ```
/// fast_exponentiation(95, 130, 7) // => '4'
/// ```
pub fn fast_exponentiation(base: &UBig, exponent: &UBig, modul: &UBig) -> UBig {
    // Sonderbedingungen der Exponentiation
    if is_one(&modul) {
        return ubig!(0);
    }
    if is_zero(&exponent) {
        return ubig!(1);
    }
    if is_one(&exponent) {
        return base.rem_euclid(modul);
    }

    // Berechnung des Zwischenschrittes mit halbiertem Exponenten.
    let base_to_square = fast_exponentiation(base, &exponent.div(2), modul);

    return if is_even(&exponent) {
        // Ist der Exponent gerade, so wird nur quadriert.
        base_to_square.pow(2).rem_euclid(modul)
    } else {
        // Ist der Exponent ungerade, wird die Basis erneut als Faktor herangezogen.
        (base_to_square.pow(2) * base).rem_euclid(modul)
    };
}

///berechnet das inverse Element in einem Restklassenring
///also zb. Das inverse zu 5 im Modulo 11 ist 9, da (5*9)%11 = 1 (neutrales Element der Multiplikation)
///
/// Faktoren im Array können negativ sein (deshalb i128) müssen dann schauen wegen bigInt
pub fn modulo_inverse(n: i128, modul: i128)-> i128 {
    let xy = [1, 0, 1];
    //berechnet aus letzten faktor den inverse
    return (modul + extended_euclidean_algorithm(modul, n, xy))%modul;
}
fn extended_euclidean_algorithm(n: i128, modul: i128, mut xy: [i128; 3]) -> i128 {
    xy.rotate_left(1);
    //println!("{},{},{} n= {} m = {}",xy[0],xy[1],xy[2],n,modul); //debug
    if modul == 0 {
        //check ob inverse existiert sonst error (durch 0 teilen XDDD ka wie error in rust geht)
        if n!=1 {
            1/modul;
        }
        return xy[0];
    }else {
        //berechnet faktoren und speichert in einem rotierendem Array
        let div: i128 = n/modul;
        xy[2]=xy[0]-(div*xy[1]);
        return extended_euclidean_algorithm(modul, n%modul,xy);
    }
}

/// Führt den Miller-Rabin-Primzahltest auf `n` durch `repeats` Runden aus.
///
/// # Argumente
/// * `n` - Die zu testende Zahl.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
///
/// # Rückgabe
/// `true`, wenn `maybe_prime` wahrscheinlich eine Primzahl ist, andernfalls `false`.
pub fn miller_rabin(p: u32, repeats: usize) -> bool {
    let mut result = true;
    for _ in 0..repeats {
        result &= miller_rabin_single(p)
    }
    result
}

/// Führt den Miller-Rabin-Primzahltest auf `n` aus.
///
/// # Argumente
/// * `n` - Die zu testende Zahl.
///
/// # Rückgabe
/// `true`, wenn `maybe_prime` wahrscheinlich eine Primzahl ist, andernfalls `false`.
fn miller_rabin_single(p: u32) -> bool {
    let mut d = p - 1;
    let mut r = 0;

    while d % 2 == 0 {
        d = d / 2;
        r += 1;
    }

    // Fun Fact:
    // Wenn man p = 221 (NICHT prim) setzt und das a manuell auf 174 setzt, kommt er
    // fälschlicherweise auf "prim" als Ergebnis.
    let a: u32 = thread_rng().gen_range(2..p - 1);
    let mut x = mod_exp(a, d, p);

    if x == 1 || x == p - 1 {
        return true;
    }
    while r > 1 {
        x = (x * x) % p;
        if x == 1 {
            return false;
        }
        if x == p - 1 {
            return true;
        }
        r -= 1;
    }

    return false;
}
