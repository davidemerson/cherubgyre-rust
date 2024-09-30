use std::fs::{OpenOptions};
use std::io::{Write, BufRead, BufReader};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use chrono::{DateTime, Utc};

static USER_FILE_PATH: &str = "users_db.txt";
static INVITE_FILE_PATH: &str = "invites_db.txt";

lazy_static! {
    static ref FILE_MUTEX: Mutex<()> = Mutex::new(());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub invite_code: String,
    pub normal_pin: String,
    pub duress_pin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub code: String,
    pub invitor_id: String, // ID of the user who created the invite
    pub count: u32, // Number of invitees registered using this invite
    pub created_at: DateTime<Utc>, // Date when invite was created
}

pub async fn save_user(user: &User) -> Result<(), std::io::Error> {
    let _guard = FILE_MUTEX.lock().await;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(USER_FILE_PATH)?;
    let user_json = serde_json::to_string(user)?;
    writeln!(file, "{}", user_json)?;
    Ok(())
}

pub async fn save_invite(invite: &Invite) -> Result<(), std::io::Error> {
    let _guard = FILE_MUTEX.lock().await;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(INVITE_FILE_PATH)?;
    let invite_json = serde_json::to_string(invite)?;
    writeln!(file, "{}", invite_json)?;
    Ok(())
}

pub async fn get_invite(code: &str) -> Option<Invite> {
    let _guard = FILE_MUTEX.lock().await;
    let file = OpenOptions::new().read(true).open(INVITE_FILE_PATH).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(invite_json) = line {
            if let Ok(invite) = serde_json::from_str::<Invite>(&invite_json) {
                if invite.code == code {
                    return Some(invite);
                }
            }
        }
    }

    None
}

pub async fn get_user_invites(user_id: &str) -> Result<Vec<Invite>, std::io::Error> {
    let _guard = FILE_MUTEX.lock().await;
    let file = OpenOptions::new().read(true).open(INVITE_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut invites = Vec::new();

    for line in reader.lines() {
        if let Ok(invite_json) = line {
            if let Ok(invite) = serde_json::from_str::<Invite>(&invite_json) {
                if invite.invitor_id == user_id {
                    invites.push(invite);
                }
            }
        }
    }

    Ok(invites)
}

pub async fn update_invite(invite: &Invite) -> Result<(), std::io::Error> {
    let _guard = FILE_MUTEX.lock().await;

    // Read existing invites
    let mut file = OpenOptions::new().read(true).open(INVITE_FILE_PATH)?;
    let mut invites: Vec<Invite> = Vec::new();
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        if let Ok(invite_json) = line {
            if let Ok(existing_invite) = serde_json::from_str::<Invite>(&invite_json) {
                if existing_invite.code == invite.code {
                    invites.push(invite.clone());
                } else {
                    invites.push(existing_invite);
                }
            }
        }
    }

    // Overwrite the file with updated invites
    file = OpenOptions::new().write(true).truncate(true).open(INVITE_FILE_PATH)?;
    for invite in invites.iter() {
        let invite_json = serde_json::to_string(&invite)?;
        writeln!(file, "{}", invite_json)?;
    }

    Ok(())
}
