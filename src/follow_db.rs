// follow_db.rs
use std::fs::{OpenOptions};
use std::io::{Write, BufRead, BufReader};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use std::io::Error;

lazy_static! {
    static ref FILE_MUTEX: Mutex<()> = Mutex::new(());
}

static FOLLOW_FILE_PATH: &str = "follows_db.txt";


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Follow {
    pub follower_id: String,
    pub followed_id: String,
}



pub async fn add_follow(follower_id: &str, followed_id: &str) -> Result<(), Error> {
    let _guard = FILE_MUTEX.lock().await;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FOLLOW_FILE_PATH)?;
    let follow = Follow {
        follower_id: follower_id.to_string(),
        followed_id: followed_id.to_string(),
    };
    let follow_json = serde_json::to_string(&follow)?;
    writeln!(file, "{}", follow_json)?;
    Ok(())
}

pub async fn remove_follow(follower_id: &str, followed_id: &str) -> Result<(), Error> {
    let _guard = FILE_MUTEX.lock().await;

    // Read existing follows
    let mut file = OpenOptions::new().read(true).open(FOLLOW_FILE_PATH)?;
    let mut follows: Vec<Follow> = Vec::new();
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        if let Ok(follow_json) = line {
            if let Ok(existing_follow) = serde_json::from_str::<Follow>(&follow_json) {
                if !(existing_follow.follower_id == follower_id && existing_follow.followed_id == followed_id) {
                    follows.push(existing_follow);
                }
            }
        }
    }

    // Overwrite the file with updated follows
    file = OpenOptions::new().write(true).truncate(true).open(FOLLOW_FILE_PATH)?;
    for follow in follows.iter() {
        let follow_json = serde_json::to_string(&follow)?;
        writeln!(file, "{}", follow_json)?;
    }

    Ok(())
}

pub async fn get_followers(user_id: &str) -> Result<Vec<String>, Error> {
    let _guard = FILE_MUTEX.lock().await;
    let file = OpenOptions::new().read(true).open(FOLLOW_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut followers = Vec::new();

    for line in reader.lines() {
        if let Ok(follow_json) = line {
            if let Ok(follow) = serde_json::from_str::<Follow>(&follow_json) {
                if follow.followed_id == user_id {
                    followers.push(follow.follower_id);
                }
            }
        }
    }

    Ok(followers)
}
