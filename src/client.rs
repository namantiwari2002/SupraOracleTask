use reqwest::header::{AUTHORIZATION};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::time::{Instant};

use rand::rngs::ThreadRng;
use rand::Rng;

use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::collections::HashMap;

use super::aggregator::{Aggregator, AggregatorClass}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Root{
    data: ReturnDataType,
    timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnDataType{
    id: String,
    symbol: String,
    #[serde(rename = "currencySymbol")]
    currency_symbol: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "rateUsd")]
    rate_usd: String,

}

pub struct APIAgent {
    prices: Vec<f64>,
    avg_price: f64,
}

impl APIAgent {
    pub async fn valueUpdate(&mut self) -> Result<(), reqwest::Error> {
        let agent = reqwest::Client::new();
        
        let response: Root= agent
            .get("https://api.coincap.io/v2/rates/bitcoin").header(AUTHORIZATION, "99bc2a7a-0ba2-4d79-8183-c1ee679591d8").send().await?.json().await?;
             self.prices.push(response.data.rate_usd.parse().unwrap());
             Ok(())
    }
    pub fn averageCalculator(&mut self){
        let total: f64 = self.prices.iter().sum();
        self.avg_price = total / self.prices.len() as f64;
    }

    pub async fn clientFactory( &mut self , times:u64) {
        let start = Instant::now();
        while start.elapsed().as_secs()<times{
        let _ = self.valueUpdate().await; 
            
        }
        self.averageCalculator();
   
         
    }
    pub fn clientKeyFactory(&mut self) -> Ed25519KeyPair  {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key pair");
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .expect("Failed to convert PKCS8 to key pair");
        key_pair
    }
    
    pub fn clientKeySign(&mut self, private_key: &Ed25519KeyPair, message: &str) -> Vec<u8> {
        let signature = private_key.sign(message.as_bytes());
        signature.as_ref().to_vec()
    }
  
    
}

pub async fn multiple_agents(times:u64){
   
    let mut handles = vec![];
    let aggregatorMain = Arc::new(Mutex::new(AggregatorClass{vec_avg_prices: Vec::new(), avg_of_avg_prices: 0.0}));
    

    for i in 1..=5 {
        let aggregator = aggregatorMain.clone();
        let handle = tokio::task::spawn(async move {
            let mut agent = APIAgent {
                prices: Vec::new(),
                avg_price: 0.0,
            };

            agent.clientFactory(times).await;

            let private_key = agent.clientKeyFactory();
            let public_key = private_key.public_key();
            let public_key_bytes = private_key.public_key().as_ref().to_vec();
            let message = "okTesting";
            let signature = agent.clientKeySign(&private_key, message);
    
            let mut temp_aggregator = aggregator.lock().unwrap();
            let isTrue = temp_aggregator.verifyClientKey(&public_key_bytes, message, &signature);
         
            if isTrue {
                println!("Agent {} Verification Successful", i);
                temp_aggregator.updateValue(agent.avg_price);

                let mut path = std::env::current_dir().unwrap().display().to_string();
                path += "/src/cache_data.txt";
                let mut file =
                    OpenOptions::new().append(true).open(path).expect("Cannot Open File");

                for ele in agent.prices.iter() {
                    let _ = file.write_all(ele.to_string().as_bytes());
                }

                let _ = file.write_all(format!(" \n Average of Agent {} is : \n", i).as_bytes());
                let _ = file.write_all(agent.avg_price.to_string().as_bytes());
            } else {
                println!("Agent {} Verification Failed", i);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Failed to await task");
    }

    aggregatorMain.lock().unwrap().updateAverage();
     println!(
         "Cache Complete! Use 'mode=read' to view results. The average price of Bitcoin in USD : {}" , 
         aggregatorMain.lock().unwrap().avg_of_avg_prices
     );

}


