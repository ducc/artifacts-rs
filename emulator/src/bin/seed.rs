use std::{io::ErrorKind, result};

use artifacts_openapi::{
    ApiNoContext, Client, ContextWrapperExt, GetAllGeItemsGeGetResponse, GetAllItemsItemsGetResponse, GetAllMapsMapsGetResponse, GetAllMonstersMonstersGetResponse, GetAllResourcesResourcesGetResponse, GetMonsterMonstersCodeGetResponse
};
use swagger::{AuthData, ContextBuilder, EmptyContext, Nullable, Push, XSpanIdString};
use tokio::io::AsyncWriteExt;

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let auth_data = AuthData::Bearer(swagger::auth::Bearer { token: std::env::var("ARTIFACTS_TOKEN").expect("missing ARTIFACTS_TOKEN env var") });

    let context: ClientContext = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        Some(auth_data),
        XSpanIdString::default()
    );

    let client = Box::new(
        Client::try_new_https("https://api.artifactsmmo.com")
            .expect("Failed to create HTTPS client"),
    );
    let mut client: Box<dyn ApiNoContext<ClientContext>> = Box::new(client.with_context(context));

    let mut page: u32 = 1;
    let mut resources = vec![];

    loop {
        let result = client
            .get_all_resources_resources_get(None, None, None, None, Some(page as i32), Some(100))
            .await?;

        match result {
            GetAllResourcesResourcesGetResponse::SuccessfullyFetchedResourcesDetails(details) => {
                if details.data.is_empty() {
                    break;
                }

                for resource in details.data {
                    resources.push(resource);
                }
            }
            GetAllResourcesResourcesGetResponse::ResourcesNotFound => {
                break;
            }
        }

        page += 1;
    }

    let mut page: u32 = 1;
    let mut maps = vec![];

    loop {
        let mut result = client
            .get_all_maps_maps_get(None, None, Some(page as i32), Some(100))
            .await?;

        match result {
            GetAllMapsMapsGetResponse::SuccessfullyFetchedMapsDetails(details) => {
                if details.data.is_empty() {
                    break;
                }

                for map in details.data {
                    maps.push(map);
                }
            }
            GetAllMapsMapsGetResponse::MapsNotFound => {
                break;
            }
        }

        page += 1;
    }

    let mut page: u32 = 1;
    let mut monsters = vec![];

    loop {
        let mut result = client
            .get_all_monsters_monsters_get(None, None, None, Some(page as i32), Some(100))
            .await?;

        match result {
            GetAllMonstersMonstersGetResponse::SuccessfullyFetchedMonstersDetails(details) => {
                if details.data.is_empty() {
                    break;
                }

                for monster in details.data {
                    monsters.push(monster);
                }
            }
            GetAllMonstersMonstersGetResponse::MonstersNotFound => {
                break;
            }
        }

        page += 1;
    }

    let mut page: u32 = 1;
    let mut items = vec![];

    loop {
        let mut result = client
            .get_all_items_items_get(
                None,
                None,
                None,
                None,
                None,
                None,
                Some(page as i32),
                Some(100),
            )
            .await?;

        match result {
            GetAllItemsItemsGetResponse::FetchItemsDetails(details) => {
                if details.data.is_empty() {
                    break;
                }

                for item in details.data {
                    items.push(item);
                }
            }
            GetAllItemsItemsGetResponse::ItemsNotFound => {
                break;
            }
        }

        page += 1;
    }

    let mut page: u32 = 1;
    let mut ge_items = vec![];

    loop {
        let mut result = client
            .get_all_ge_items_ge_get(
                Some(page as i32),
                Some(100),
            )
            .await?;

        match result {
            GetAllGeItemsGeGetResponse::FetchGrandExchangeItemsDetails(details) => {
                if details.data.is_empty() {
                    break;
                }

                for item in details.data {
                    ge_items.push(item);
                }
            }
            GetAllGeItemsGeGetResponse::ItemNotFound => {
                break;
            }
        }

        page += 1;
    }

    match tokio::fs::create_dir(".seed_data").await {
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        r => r,
    }?;

    let ts = chrono::Utc::now().to_string();
    let mut timestamp_file = tokio::fs::File::create(".seed_data/.timestamp").await?;
    timestamp_file.write_all(ts.as_bytes()).await?;

    let serialized = serde_json::to_vec_pretty(&resources)?;
    let mut file = tokio::fs::File::create(".seed_data/resources.json").await?;
    file.write_all(&serialized).await?;

    let serialized = serde_json::to_vec_pretty(&maps)?;
    let mut file = tokio::fs::File::create(".seed_data/maps.json").await?;
    file.write_all(&serialized).await?;

    let serialized = serde_json::to_vec_pretty(&monsters)?;
    let mut file = tokio::fs::File::create(".seed_data/monsters.json").await?;
    file.write_all(&serialized).await?;

    let serialized = serde_json::to_vec_pretty(&items)?;
    let mut file = tokio::fs::File::create(".seed_data/items.json").await?;
    file.write_all(&serialized).await?;

    let serialized = serde_json::to_vec_pretty(&ge_items)?;
    let mut file = tokio::fs::File::create(".seed_data/ge_items.json").await?;
    file.write_all(&serialized).await?;

    Ok(())
}
