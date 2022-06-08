#[cfg(test)]
mod tests {
    use test_log::test;
    use workspaces::network::DevAccountDeployer;

    pub mod ext {
        include!("../gen/abi.rs");
    }

    #[test(tokio::test)]
    async fn it_works() -> anyhow::Result<()> {
        let worker = workspaces::sandbox().await?;
        let contract = worker
            .dev_deploy(include_bytes!(
                "../../target/wasm32-unknown-unknown/release/adder.wasm"
            ))
            .await?;

        let contract = ext::ExtAdder { contract: contract };
        let res = contract.add(&worker, vec![3, 4], vec![2, 1]).await?;
        println!("Result: {:?}", res);

        Ok(())
    }
}
