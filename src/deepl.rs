use std::time::SystemTime;

use rand;
use rand::Rng;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
    pub id: u64,
}

impl Payload {
    pub fn new(sentence: String, source: String, target: String) -> Self {
        let mut rng = rand::thread_rng();

        Payload {
            jsonrpc: "2.0".into(),
            method: "LMT_handle_jobs".into(),
            params: Params::new(sentence, source, target),
            id: rng.gen_range(0, 10000) * 10000,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub jobs: Vec<Job>,
    pub lang: Lang,
    pub priority: u64,
    pub timestamp: u64,
}

impl Params {
    pub fn new(sentence: String, source: String, target: String) -> Self {
        Params {
            jobs: vec![Job::new(sentence)],
            lang: Lang::new(source, target),
            priority: 1,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                * 1000,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub kind: String,
    pub raw_en_sentence: String,
    pub raw_en_context_before: Vec<String>,
    pub raw_en_context_after: Vec<String>,
    pub preferred_num_beams: u64,
}

impl Job {
    pub fn new(sentence: String) -> Self {
        Job {
            kind: "default".into(),
            raw_en_sentence: sentence,
            raw_en_context_before: Vec::new(),
            raw_en_context_after: Vec::new(),
            preferred_num_beams: 4,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Lang {
    pub user_preferred_langs: Vec<String>,
    pub source_lang_computed: String,
    pub target_lang: String,
}

impl Lang {
    pub fn new(source: String, target: String) -> Self {
        Lang {
            user_preferred_langs: vec!["IT", "NL", "PL", "RU", "PT", "ES", "DE", "FR", "EN"]
                .drain(..)
                .map(|x| x.into())
                .collect(),
            source_lang_computed: source,
            target_lang: target,
        }
    }
}
