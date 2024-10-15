// duress_db.rs
use std::fs::{OpenOptions};
use std::io::{Write, BufRead, BufReader};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use std::io::Error;
// duress_db.rs
use crate::duress_handlers::MapInfo;

lazy_static! {
	static ref FILE_MUTEX: Mutex<()> = Mutex::new(());
}

static DURESS_FILE_PATH: &str = "duress_db.txt";
static PREFERENCES_FILE_PATH: &str = "preferences_db.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
	pub broadcast_duress: bool,
	pub receive_duress_broadcasts: bool,
}

// Log a duress event
pub async fn log_duress_event(
	user_id: &str,
	duress_type: &str,
	message: &str,
	timestamp: &str,
) -> Result<(), Error> {
	let _guard = FILE_MUTEX.lock().await;
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(DURESS_FILE_PATH)?;
	let duress_entry = format!("{}|{}|{}|{}\n", user_id, duress_type, message, timestamp);
	writeln!(file, "{}", duress_entry)?;
	Ok(())
}

// Cancel a duress event
pub async fn cancel_duress(user_id: &str) -> Result<(), Error> {
	let _guard = FILE_MUTEX.lock().await;

	// Placeholder: Logic to cancel duress for the user
	// TODO: Adjust behavior based on your application's needs
	Ok(())
}

// Enable test mode for duress
pub async fn enable_test_mode(user_id: &str) -> Result<(), Error> {
	let _guard = FILE_MUTEX.lock().await;

	// Placeholder: Logic to enable test mode
	Ok(())
}

// Retrieve map information for followed users
pub async fn get_followed_users_map_info(user_id: &str) -> Result<Vec<MapInfo>, Error> {
	// Placeholder: Retrieve last check-in locations and duress status
	// TODO: Integrate with real map data storage
	let map_info = vec![
		MapInfo {
			user_id: "follower1".to_string(),
			username: "Follower One".to_string(),
			location: "Location1".to_string(),
			duress: false,
			last_checkin: "2024-10-05T11:57:33.803Z".to_string(),
		},
		// Additional entries as necessary
	];
	Ok(map_info)
}

// Get user preferences
pub async fn get_user_preferences(user_id: &str) -> Result<UserPreferences, Error> {
	let _guard = FILE_MUTEX.lock().await;

	let file = OpenOptions::new().read(true).open(PREFERENCES_FILE_PATH)?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
		if let Ok(preference_json) = line {
			if let Ok(preference) = serde_json::from_str::<UserPreferences>(&preference_json) {
				return Ok(preference);
			}
		}
	}

	Ok(UserPreferences {
		broadcast_duress: true,
		receive_duress_broadcasts: true,
	})
}

// Update user preferences
pub async fn update_user_preferences(
	user_id: &str,
	preferences: UserPreferences
) -> Result<(), Error> {
	let _guard = FILE_MUTEX.lock().await;

	// Placeholder: Logic to update preferences in a real storage
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(PREFERENCES_FILE_PATH)?;
	let preferences_json = serde_json::to_string(&preferences)?;
	writeln!(file, "{}", preferences_json)?;
	Ok(())
}
