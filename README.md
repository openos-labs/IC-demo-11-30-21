# demo

> *一步步实现逻辑并在主网部署你的 demo.*

- [demo](#demo)
  - [motoko demo (15 min)](#motoko-demo-15-min)
  - [rust demo (5 min)](#rust-demo-5-min)
- [reference](#reference)

## motoko demo (15 min)

1. `dfx new demo_mo`
2. delete asset & motify config `dfx.json`
3. add code and explain logic and related libraries
4. new a canister in **IC**, add config `canister_ids.json`
5. deploy on **IC**
6. get result in explore, https://fesla-yiaaa-aaaah-aa4na-cai.raw.ic0.app/, dfx query
   ```sh
   dfx canister --network ic --no-wallet call demo_mo allBalances --query
   
   dfx identity use alice
   dfx identity get-principal
   ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae

   dfx identity use bob
   dfx identity get-principal
   yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe

   dfx identity use icp
   dfx identity get-principal
   yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae
   ```
7. dfx call (userA -> alice 100, alice -> bob 50, bob -> userA 200 (failed))
   ```sh
   dfx canister --network ic --no-wallet call demo_mo transfer '(principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae", 100)'
   dfx canister --network ic --no-wallet call demo_mo transfer '(principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe", 50)'
   dfx canister --network ic --no-wallet call demo_mo transfer '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae", 200)
8. get result in explore, dfx query
   ```sh
   dfx canister --network ic --no-wallet call demo_mo allBalances --query
   ```
9.  delete allBalance, upgrade, get result in explore, https://b4e6x-kaaaa-aaaah-aa4uq-cai.raw.ic0.app/
   ```sh
   sudo dfx build --network ic
   sudo dfx canister --no-wallet --network ic install demo_mo -m=upgrade 
   ```

## rust demo (5 min)

1. `cargo new demo_rs`
2. add config `dfx.json`
3. add code and explain logic and related libraries
4. new a canister in **IC**, add config `canister_ids.json`
5. deploy on **IC**
   ```sh
   dfx canister --no-wallet --network ic install demo_rs
   ```
6. get result in explore, dfx query 
   ```sh
   dfx canister --network ic --no-wallet call demo_rs transfer '(principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae", 100)'
   ```


# reference
* https://smartcontracts.org/docs/language-guide/motoko.html
* https://github.com/rocklabs-io/ic-token
* https://github.com/dfinity/motoko-base
* https://github.com/dfinity/cdk-rs
* https://docs.rs/ic-cdk/0.3.3/ic_cdk/
* https://github.com/dfinity/candid
* https://docs.rs/candid/0.7.8/candid/* 
