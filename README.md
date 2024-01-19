
# SupraOracle - Internship Task

This project is a component of the SupraOracle Rust internship task.

Its primary objective is to fetch the real-time price of Bitcoin (BTC) in USD using the CoinCap API . The implementation involves utilizing five agents to ensure efficient and reliable data retrieval.

CoinCap API used : ```https://api.coincap.io/v2/rates/bitcoin```

### Setting Up Project
First clone the project then build the cargo and finally enjoy project!

Clone repo : ```git clone```

Build Cargo : ```cargo build```

### Commands to run project :
The project can be executed in two modes: 

Cache Mode : To run the agents and collect data in a file 'cache_data.txt'

 ```cargo run -- --mode=cache --times=<number_of_time>```

Read Mode : To read data from the file 

```cargo run -- --mode=read```

### Code Structure

1.  client.rs : It countaints code to create agents and initialize them.

2.  aggregator.rs : All the aggregation operations are done here, client utilises this to calcaulate values and averages.

3.  main.rs : Binds everything and also contains function to handle read mode of the program.

4. cache_data.txt : It stores the data values collected by the agents and it content get fetched with the read mode.

