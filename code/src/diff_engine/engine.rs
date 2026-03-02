use std::collections::HashMap;

use entity::intermidiate;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

pub struct ChurnAndRetentionData {
    pub sdk_id: i64,
    pub retention: u64,
    pub churn: HashMap<i64, u64>,
}
pub struct CurnAndRetentionMap {
    pub map: HashMap<i64, ChurnAndRetentionData>,
}
// this assumes that both baseline and successor are divided up by the app_id
// this determines a single point of data for each sdk per ap. To increment or decrement
impl CurnAndRetentionMap {
    fn calculate_churns_and_retentions(
        &mut self,
        baseline: HashMap<i64, &entity::app_sdk::Model>,
        successor: HashMap<i64, &entity::app_sdk::Model>,
        db: &DatabaseConnection,
    ) -> () {
        // todo: this should maybe return a pointer to a database
        for (sdk_id_baseline, app_sdk_baseline) in &baseline {
            // we first need to check that if data for both points exists, if not we skip it for now since there is no data,
            // todo : record this irregularity
            if !successor.contains_key(sdk_id_baseline) {
                continue;
            }

            // this should never fail due to above check
            let successor_app_sdk = successor.get(sdk_id_baseline).unwrap();

            match (app_sdk_baseline.installed, successor_app_sdk.installed) {
                // this is a signal of retention
                (true, true) => {
                    let active_model = intermidiate::ActiveModel {
                        from_sdk: Set(sdk_id_baseline.to_owned()),
                        to_sdk: Set(successor_app_sdk.sdk_id),
                        ..Default::default()
                    };
                    let _ = active_model.save(db);
                }

                (true, false) => {
                    let active_model = intermidiate::ActiveModel {
                        from_sdk: Set(sdk_id_baseline.to_owned()),
                        to_sdk: Set(successor_app_sdk.sdk_id),
                        ..Default::default()
                    };
                    let _ = active_model.save(db);
                }
                (false, true) => {
                    // this will already be processed when processing the sdk which gained,
                    //since it requires a  mirroring (true,false) match
                    // todo: handling it here could help with performance
                    // for now, skip
                    continue;
                }
                (false, false) => continue,
            }
        }
    }
}
