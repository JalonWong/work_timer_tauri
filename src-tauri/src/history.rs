use serde::{Deserialize, Serialize};
use sled::{Db, IVec};
use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct History {
    db: Db,
}

impl History {
    pub fn new() -> Self {
        let mut path = crate::settings::get_config_dir();
        path.push("history_db");
        Self {
            db: sled::open(&path).unwrap(),
        }
    }

    pub fn add_record(&mut self, start_time: &SystemTime, duration: u64, label: &str) {
        let key = Self::to_key(start_time);
        let record = RecordTmp {
            d: duration,
            l: label.to_string(),
        };
        self.db
            .insert(key, toml::to_string(&record).unwrap().as_bytes())
            .ok();
        self.db.flush().ok();
    }

    pub fn get_records(&self, start: &SystemTime, end: &SystemTime, reverse: bool) -> Vec<Record> {
        let start = Self::to_key(start);
        let end = Self::to_key(end);
        let mut rst = Vec::new();
        if reverse {
            for (key, value) in self.db.range(start..end).rev().flatten() {
                if let Some(record) = Self::to_record(key, value) {
                    rst.push(record);
                }
            }
        } else {
            for (key, value) in self.db.range(start..end).flatten() {
                if let Some(record) = Self::to_record(key, value) {
                    rst.push(record);
                }
            }
        }

        // println!("{:?}", rst);
        rst
    }

    pub fn remove(&mut self, key: u64) {
        self.db.remove(key.to_be_bytes()).ok();
        self.db.flush().ok();
    }

    pub fn modify_record(&mut self, key: u64, duration: u64, label: &str) {
        self.db
            .fetch_and_update(key.to_be_bytes(), |value| {
                if let Some(value) = value
                    && let Ok(mut record) =
                        toml::from_str::<RecordTmp>(std::str::from_utf8(value).unwrap())
                {
                    record.l = label.to_string();
                    record.d = duration;
                    return Some(toml::to_string(&record).unwrap().into_bytes());
                }
                None
            })
            .ok();
        self.db.flush().ok();
    }

    pub fn export_to_csv(&self, file_path: PathBuf) {
        let mut writer = csv::Writer::from_path(file_path).unwrap();
        for record in self.get_records(&SystemTime::UNIX_EPOCH, &SystemTime::now(), true) {
            let dt = record
                .start_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            writer
                .write_record(&[
                    dt.as_secs().to_string(),
                    record.duration.to_string(),
                    record.label,
                ])
                .unwrap();
        }
        writer.flush().unwrap();
    }

    fn to_record(key: IVec, value: IVec) -> Option<Record> {
        if let Ok(value) = std::str::from_utf8(value.as_ref())
            && let Ok(t) = toml::from_str::<RecordTmp>(value)
        {
            let array: [u8; 8] = key.as_ref().try_into().unwrap();
            let start_time_u64 = u64::from_be_bytes(array);
            let start_time = UNIX_EPOCH + Duration::from_secs(start_time_u64);
            return Some(Record {
                key: start_time_u64,
                start_time,
                duration: t.d,
                label: t.l,
            });
        }
        None
    }

    fn to_key(start_time: &SystemTime) -> [u8; 8] {
        start_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_be_bytes()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Record {
    pub key: u64,
    pub start_time: SystemTime,
    pub duration: u64,
    pub label: String,
}

#[derive(Deserialize, Serialize)]
struct RecordTmp {
    d: u64,
    l: String,
}
