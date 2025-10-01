//! Platform detection and initialization
//!
//! Determines the current platform and provides platform-specific initialization.

/// Supported platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Platform {
	Windows,
	MacOS,
	Linux,
	Android,
	iOS,
	Unknown,
}

impl Platform {
	/// Detect the current platform
	pub fn current() -> Self {
		#[cfg(target_os = "windows")]
		return Platform::Windows;

		#[cfg(target_os = "macos")]
		return Platform::MacOS;

		#[cfg(target_os = "linux")]
		return Platform::Linux;

		#[cfg(target_os = "android")]
		return Platform::Android;

		#[cfg(target_os = "ios")]
		return Platform::iOS;

		#[cfg(not(any(
			target_os = "windows",
			target_os = "macos",
			target_os = "linux",
			target_os = "android",
			target_os = "ios"
		)))]
		Platform::Unknown
	}

	/// Get platform name as string
	pub fn name(&self) -> &'static str {
		match self {
			Platform::Windows => "Windows",
			Platform::MacOS => "macOS",
			Platform::Linux => "Linux",
			Platform::Android => "Android",
			Platform::iOS => "iOS",
			Platform::Unknown => "Unknown",
		}
	}

	/// Check if platform is supported
	pub fn is_supported(&self) -> bool {
		!matches!(self, Platform::Unknown)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_current_platform() {
		let platform = Platform::current();
		assert!(platform.is_supported());
		assert!(!platform.name().is_empty());
	}

	#[test]
	fn test_platform_names() {
		assert_eq!(Platform::Windows.name(), "Windows");
		assert_eq!(Platform::MacOS.name(), "macOS");
		assert_eq!(Platform::Linux.name(), "Linux");
		assert_eq!(Platform::Android.name(), "Android");
		assert_eq!(Platform::iOS.name(), "iOS");
	}
}
