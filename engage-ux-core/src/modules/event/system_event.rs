use serde::{Deserialize, Serialize};

/// A system event, such as online/offline status or battery changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
	/// The system has gone online.
	Online,
	/// The system has gone offline.
	Offline,
	/// The battery level has changed.
	BatteryChange(f32),
	/// The device orientation has changed.
	OrientationChange(Orientation),
	/// A device has been connected or disconnected.
	DeviceConnected(String),
	/// A device has been disconnected.
	DeviceDisconnected(String),
}

/// The orientation of the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
	/// The device is in portrait orientation.
	Portrait,
	/// The device is in landscape orientation.
	Landscape,
	/// The device is in inverted portrait orientation.
	PortraitInverted,
	/// The device is in inverted landscape orientation.
	LandscapeInverted,
}
