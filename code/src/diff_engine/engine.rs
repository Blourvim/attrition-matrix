use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq)]
enum Status {
    // Retention and attrition are the primary statues to be rendered on the matrix
    Retention(i64),
    // Attrition(HashMap<i64, i64>),
    // these are secondary, which won't be rendered, but we may want to represent them regardless at a later date
    //Addition,
    //MissingSuccessor,
    //MissingPast,
    //RemovedWithoutReplacement,
}

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
    ) -> Vec<i64> {
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
                    //todo: dry duplicate work
                    match self.map.get_mut(sdk_id_baseline) {
                        Some(val) => val.retention += 1,
                        None => {
                            self.map.insert(
                                *sdk_id_baseline,
                                ChurnAndRetentionData {
                                    sdk_id: *sdk_id_baseline,
                                    retention: 1,
                                    churn: HashMap::new(),
                                },
                            );
                        }
                    }
                }
                (true, false) => {
                    todo!("this is attrition")
                }
                (false, true) => {
                    // this will already be processed when processing the sdk which gained,
                    //since it requires a  mirroring (true,false) match
                    // todo: handling it here could help with performance
                    todo!("this is addition")
                }
                (false, false) => continue,
            }
        }
        todo!()
    }
}
