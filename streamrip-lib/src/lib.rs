pub mod base;
pub mod clients;
pub mod constants;
pub mod metadata;
mod utils;

use ahash::AHashMap;
use std::error::Error;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;
pub type JsonMap = AHashMap<String, serde_json::Value>;

#[cfg(test)]
mod tests {
    use crate::base::*;
    use crate::clients::qobuz::*;
    use crate::clients::tidal::*;
    use log::*;
    use rstest::{fixture, rstest};
    use std::fs;
    use tokio;
    use tokio::runtime::Runtime;

    #[fixture]
    fn qobuz_client_new(runtime: Runtime) -> QobuzClient {
        let res = setup_logger();
        assert!(matches!(res, Ok(_)));

        let creds = fs::read_to_string("credentials.txt").unwrap();
        let app_id = "950096963".to_string();
        let secrets = "979549437fcc4a3faad4867b5cd25dcb".to_string();
        // "10b251c286cfbf64d6b7105f253d9a2e;979549437fcc4a3faad4867b5cd25dcb".to_string();

        runtime.block_on(async move {
            QobuzClient::with_tokens(
                vec![
                // TODO: fill in creds
                ],
                app_id,
                secrets,
            )
            .await
            .unwrap()
        })
    }

    #[rstest]
    fn tidal_login_url(runtime: Runtime) {
        let res = setup_logger();
        assert!(res.is_ok());

        let res = runtime.block_on(async { TidalClient::get_login_url().await });

        assert!(res.is_ok());
    }

    #[fixture]
    fn qobuz_client(runtime: Runtime, mut qobuz_client_new: QobuzClient) -> QobuzClient {
        runtime.block_on(async move {
            qobuz_client_new.login().await.unwrap();
            qobuz_client_new
        })
    }

    fn setup_logger() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            .chain(fern::log_file("output.log")?)
            .apply()?;

        Ok(())
    }

    #[fixture]
    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    }

    #[rstest]
    fn fetch_qobuz_app_id_and_secrets(runtime: Runtime) {
        let res = runtime.block_on(async { get_app_id_and_secrets().await });
        println!("{:?}", res);
        assert!(matches!(res, Ok(_)));
    }

    #[rstest]
    fn qobuz_get_file_url(runtime: Runtime, qobuz_client: QobuzClient) {
        let client = qobuz_client;
        let res = runtime.block_on(async { client.get_file_url("19512574", 3).await });
        println!("result: {:?}", res);
        assert!(res.is_ok());
    }

    #[rstest]
    fn qobuz_get_album_meta(runtime: Runtime, qobuz_client: QobuzClient) {
        // Ariana Grande - My Everytging
        // https://open.qobuz.com/album/spyml3fraxloa

        let client = qobuz_client;
        let res = runtime.block_on(async {
            client
                .get_metadata("spyml3fraxloa", MediaType::Album, None, None)
                .await
        });

        assert!(res.is_ok());
    }

    #[rstest]
    fn qobuz_get_track_meta(runtime: Runtime, qobuz_client: QobuzClient) {
        // Ariana Grande - Problem
        // https://open.qobuz.com/track/54310620

        let client = qobuz_client;
        let res = runtime.block_on(async {
            client
                .get_metadata("54310620", MediaType::Track, None, None)
                .await
        });

        assert!(res.is_ok());
    }

    #[rstest]
    fn qobuz_search_album(runtime: Runtime, qobuz_client: QobuzClient) {
        let res = runtime.block_on(async {
            qobuz_client
                .search("rumours", MediaType::Album, None, None)
                .await
        });
        info!("{:?}", res);
        assert!(res.is_ok());
    }
}
