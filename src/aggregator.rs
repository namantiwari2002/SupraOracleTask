use serde::{Serialize, Deserialize};
use rand::rngs::OsRng;
use rand::Rng;


// use ring::rand::{SystemRandom, SecureRandom};
// use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::collections::HashMap;


// use rsa::Hash::Hashes;


pub trait Aggregator {
    fn updateAverage(&mut self);
    fn updateValue(&mut self, value: f64);
    fn verifyClientKey(&mut self,
        public_key: &[u8],
        message: &str,
        signature: &[u8],) -> bool;
}

pub struct AggregatorClass{
    pub vec_avg_prices: Vec<f64>,
    pub avg_of_avg_prices: f64,
}

impl Aggregator for AggregatorClass {
       
        fn updateAverage(&mut self){
           let t: f64 = self.vec_avg_prices.iter().sum();
           self.avg_of_avg_prices = t / self.vec_avg_prices.len() as f64;
       }
               
         fn updateValue(&mut self, value: f64){
               self.vec_avg_prices.push(value);

        }
        fn verifyClientKey( &mut self,
            public_key: &[u8],
            message: &str,
            signature: &[u8],) -> bool {
                let public_key =
                ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
            let result = public_key.verify(message.as_bytes(), signature);
            result.is_ok()
  }
   
}




