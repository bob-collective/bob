// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use std::io::Read;

use ethabi::{ethereum_types::U256, ParamType, Token};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


// pub mod global {
//     // #[cfg(not(feature = "std"))]
//     // use alloc::{vec, vec::Vec};
//     use core::ops::Deref;
//     use secp256k1::{ffi::types::AlignedType, AllPreallocated, Secp256k1};
//     // this is what lazy_static uses internally
//     use spin::Once;
//
//     pub struct GlobalContext {
//         __private: (),
//     }
//
//     pub static SECP256K1: &GlobalContext = &GlobalContext { __private: () };
//
//     impl Deref for GlobalContext {
//         type Target = Secp256k1<AllPreallocated<'static>>;
//
//         fn deref(&self) -> &Self::Target {
//             static ONCE: Once<()> = Once::new();
//             static mut BUFFER: Vec<AlignedType> = vec![];
//             static mut CONTEXT: Option<Secp256k1<AllPreallocated<'static>>> = None;
//             ONCE.call_once(|| unsafe {
//                 BUFFER = vec![AlignedType::zeroed(); Secp256k1::preallocate_size()];
//                 let ctx = Secp256k1::preallocated_new(&mut BUFFER).unwrap();
//                 CONTEXT = Some(ctx);
//             });
//             unsafe { CONTEXT.as_ref().unwrap() }
//         }
//     }
// }
//
// fn new_deposit_public_key_with_secret(&self, secret_key: &[u8; 32])  {
//     use secp256k1::{PublicKey as Secp256k1PublicKey};
//
//     let mut public_key = Secp256k1PublicKey::from_slice(&self.0).unwrap();
//     // D = V * c
//     public_key.mul_assign(global::SECP256K1, secret_key).unwrap();
// }

pub const PUBLIC_KEY_SIZE: usize = 33;

#[derive(Debug, PartialEq)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_SIZE]);
use libsecp256k1::*;

pub type SecureID = [u8;32];
use risc0_zkvm::sha;
impl PublicKey {
    #[allow(unused)]
    fn new_secret_key(&self, secure_id: SecureID) -> [u8; 32] {
        use sha::Sha256;
        let bytes = [&self.0[..], &secure_id[..]].concat();
        
        let digest = sha::Impl::hash_bytes(&bytes);
        let bytes = digest.as_bytes();
        bytes.try_into().unwrap()
    }

    pub fn new_deposit_public_key(&self, secure_id: SecureID) -> Self {
        self.new_deposit_public_key_with_secret(&self.new_secret_key(secure_id))
    }
// 
    fn new_deposit_public_key_with_secret(&self, secret_key: &[u8; 32]) -> Self {
        let mut public_key = libsecp256k1::PublicKey::parse_compressed(&self.0).unwrap();
        let secret_key = libsecp256k1::SecretKey::parse_slice(secret_key).unwrap();
        // // D = V * c
        public_key.tweak_mul_assign(&secret_key).unwrap();
        Self(public_key.serialize_compressed())
    }

}


#[test]
fn test_new_deposit_public_key_static() {
    // bcrt1qzrkyemjkaxq48zwlnhxvear8fh6lvkwszxy7dm
    let old_public_key = PublicKey([
        2, 123, 236, 243, 192, 100, 34, 40, 51, 111, 129, 130, 160, 64, 129, 135, 11, 184, 68, 84, 83, 198, 234,
        196, 150, 13, 208, 86, 34, 150, 10, 59, 247,
    ]);

    let secret_key = &[
        137, 16, 46, 159, 212, 158, 232, 178, 197, 253, 105, 137, 102, 159, 70, 217, 110, 211, 254, 82, 216, 4,
        105, 171, 102, 252, 54, 190, 114, 91, 11, 69,
    ];

    // bcrt1qn9mgwncjtnavx23utveqqcrxh3zjtll58pc744
    let new_public_key = old_public_key.new_deposit_public_key_with_secret(secret_key);

    assert_eq!(
        new_public_key,
        PublicKey([
            2, 151, 202, 113, 10, 9, 43, 125, 187, 101, 157, 152, 191, 94, 12, 236, 133, 229, 16, 233, 221, 52,
            150, 183, 243, 61, 110, 8, 152, 132, 99, 49, 189,
        ])
    );
}
// 
// fn sign_something() {
//     // use libsecp256k1::Secp256k1;
//     //     let secp256k1 = Secp256k1::new();
//     //
//     let message_arr = [6u8; 32];
//     //     let (secp_privkey, secp_pubkey) = secp256k1.generate_keypair(&mut thread_rng());
//     //
//     //     let secp_message = SecpMessage::from_slice(&message_arr).unwrap();
//     //     let pubkey_a = secp_pubkey.serialize_uncompressed();
//     //     assert_eq!(pubkey_a.len(), 65);
//     //     let pubkey = PublicKey::parse(&pubkey_a).unwrap();
//     //     let mut seckey_a = [0u8; 32];
//     //     for i in 0..32 {
//     //         seckey_a[i] = secp_privkey[i];
//     //     }
//     let key = [1u8; 32];
//     let seckey = SecretKey::parse(&key).unwrap();
//     let message = Message::parse(&message_arr);
// 
//     let (sig, recid) = libsecp256k1::sign(&message, &seckey);
// }
fn main() {
    // test_new_deposit_public_key_static();
    // sign_something();
    // Read data sent from the application contract.

    let input_bytes = hex::decode("000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000021027becf3c0642228336f8182a04081870bb8445453c6eac4960dd05622960a3bf70000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200101010101010101010101010101010101010101010101010101010101010101").unwrap();
    // let mut input_bytes = Vec::<u8>::new();
    // env::stdin().read_to_end(&mut input_bytes).unwrap();


    // Type array passed to `ethabi::decode_whole` should match the types encoded in
    // the application contract.
    let input = ethabi::decode(&[ParamType::Bytes, ParamType::Bytes], &input_bytes).unwrap();

    let public_key = PublicKey(input[0].clone().into_bytes().unwrap().try_into().unwrap());
    let secure_id = input[1].clone().into_bytes().unwrap().try_into().unwrap();
    

    // let public_key = PublicKey([
    //     2, 123, 236, 243, 192, 100, 34, 40, 51, 111, 129, 130, 160, 64, 129, 135, 11, 184, 68, 84, 83, 198, 234,
    //     196, 150, 13, 208, 86, 34, 150, 10, 59, 247,
    // ]);
    // let secure_id = [1u8; 32];

    let new_public_key = public_key.new_deposit_public_key(secure_id);
    
    // Commit the journal that will be received by the application contract.
    // Encoded types should match the args expected by the application callback.
    env::commit_slice(&ethabi::encode(&[Token::Bytes(new_public_key.0.to_vec())]));
}
