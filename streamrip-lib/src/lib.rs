use tokio;

mod clients;
#[cfg(test)]
mod tests {
    use crate::clients::qobuz;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn app_id_secrets() {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { qobuz::get_app_id_and_secrets().await });
        println!("OUTPUT: {:?}", res);
        assert_eq!(true, false);
    }
}

pub fn hello() {
    println!("Hello from the lib!");
}
