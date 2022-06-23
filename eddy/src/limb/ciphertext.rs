use crate::decryption_share::Verified;
use crate::limb::DecryptionShare;
use ark_ff::One;

/// an Elgamal ciphertext (c1, c2).
#[derive(Clone, Copy)]
pub struct Ciphertext {
    pub(crate) c1: decaf377::Element,
    pub(crate) c2: decaf377::Element,
}

// compute the lagrange coefficient for the participant given by `participant_index` in the set of
// participants given by participant_indices
fn lagrange_coefficient(participant_index: u32, participant_indices: Vec<u32>) -> decaf377::Fr {
    participant_indices
        .iter()
        .filter(|x| **x != participant_index)
        .fold(decaf377::Fr::one(), |acc, x| {
            let n = decaf377::Fr::from(*x);
            let i = decaf377::Fr::from(participant_index);

            acc * (n / (n - i))
        })
}

impl Ciphertext {
    pub fn decrypt(&self, shares: Vec<&DecryptionShare<Verified>>) -> decaf377::Element {
        let indices = shares
            .iter()
            .map(|s| s.participant_index)
            .collect::<Vec<_>>();

        let coeffs = indices
            .iter()
            .map(|i| lagrange_coefficient(*i, indices))
            .collect::<Vec<_>>();

        let d = shares
            .iter()
            .zip(coeffs)
            .fold(decaf377::Element::default(), |d, (share, coeff)| {
                d + share.decryption_share * coeff
            });

        -d + self.c2
    }
}
