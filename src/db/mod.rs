use crate::configuration::DatabaseConfig;
use crate::error::AppError;
use crate::handler::ServiceModel;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};

#[derive(Clone, Debug)]
pub struct DbInstance {
    pub client: Client,
    pub database: Database,
    pub collection: Collection<ServiceModel>,
}

impl DbInstance {
    pub async fn db_init(config: DatabaseConfig) -> Result<DbInstance, Error> {
        let mut client_options = ClientOptions::parse(config.get_mongouri()).await?;
        client_options.app_name = Some(env!("CARGO_PKG_NAME").to_string());
        client_options.min_pool_size = Some(*config.get_min_pool_size());
        client_options.max_pool_size = Some(*config.get_max_pool_size());
        let client = Client::with_options(client_options)?;
        let db = client.database(config.get_db_name());
        let collection = db.collection::<ServiceModel>(config.get_collection_name());
        Ok(Self {
            client: client,
            database: db,
            collection: collection,
        })
    }

    pub async fn is_service_not_exist(&self, svc: &ServiceModel) -> Result<bool, AppError> {
        let filter = doc! {
            "label": svc.get_label(),
            "gcp_name": svc.get_gcp_name(),
            "namespace" : svc.get_namespace()
        };
        let exist: Vec<ServiceModel> = self
            .collection
            .find(filter, None)
            .await?
            .try_collect()
            .await?;
        Ok(exist.is_empty())
    }

    pub async fn add_service(&self, svc: &ServiceModel) -> Result<ServiceModel, AppError> {
        let id = self.collection.insert_one(svc, None).await?;
        Ok(self
            .collection
            .find_one(doc! {"_id": id.inserted_id}, None)
            .await?
            .unwrap())
    }
}
