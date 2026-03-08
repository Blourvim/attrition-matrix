use sea_orm::{ConnectOptions, Database, DbConn};

pub enum DbSelector {
    Baseline,
    Successor,
    Intermediate,
}

pub async fn get_db(db_selection: DbSelector) -> DbConn {
    let options: ConnectOptions = match db_selection {
        // This is overly verbose, but different databases could have different optio
        DbSelector::Baseline => {
            let baseline_db_url = std::env::var("BASELINE_DB_URL").unwrap();
            let opt = ConnectOptions::new(baseline_db_url);
            opt
        }
        DbSelector::Successor => {
            let successor_db_url = std::env::var("SUCCESSOR_DB_URL").unwrap();
            let opt = ConnectOptions::new(successor_db_url);
            opt
        }
        DbSelector::Intermediate => {
            let intermediate_db_url = std::env::var("INTERMEDIATE_DB_URL").unwrap();
            let mut opt = ConnectOptions::new(intermediate_db_url);
            // this is so that the in memory db is consistent otherwise we may connect to an empty in memory db
            opt.max_connections(1);
            opt
        }
    };
    let connection = Database::connect(options)
        .await
        .expect("failed to initiate initer_db");
    connection
}
