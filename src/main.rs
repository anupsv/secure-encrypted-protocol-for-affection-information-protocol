use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, BigUint};
use rsa::traits::PublicKeyParts;
use rsa::traits::PrivateKeyParts;
use rand::rngs::OsRng;
use num_traits::One;

fn main() {
    let mut rng = OsRng;

    // Step 1: Alice generates an RSA key pair
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Step 2: Alice generates two residues x and y
    let x = BigUint::one(); // Representing '0' with padding
    let y = BigUint::from(2u32); // Representing '1' (Alice's response Yes)

    // Step 3: Alice sends x^3 and y^3 to Bob
    
    let x_cubed = &x.modpow(&BigUint::from(3u32), public_key.n());
    let y_cubed = &y.modpow(&BigUint::from(3u32), &public_key.n());

    // Bob's part
    // Step 4: Bob chooses a response and sends back a value
    let bob_response = true; // Let's say Bob's response is Yes
    let r = BigUint::from(5u32); // Random residue r
    let bob_value = if bob_response { y_cubed } else { x_cubed } * r.modpow(&BigUint::from(3u32), &public_key.n());
    let bob_value = bob_value % public_key.n();
    //
    // // Step 5: Alice calculates cube root
    // In a real scenario, this should be done securely
    let alice_value = bob_value.modpow(&private_key.d(), &public_key.n());
    //
    // // Step 6: Bob divides by r and learns the logical AND of their responses
    let final_result = alice_value / r;
    
    println!("Final result: {}", final_result);
}
