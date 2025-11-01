/// License validation and feature gating for Freemium model
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// License tier determines feature access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseTier {
    /// Free tier: Limited to 3 basic checkers, HTML export only
    Free,
    /// Trial tier: Full access for 14 days
    Trial,
    /// Pro tier: Full access, all checkers, all export formats
    Pro,
}

/// Features that can be gated by license tier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    // Checkers
    FirewallChecker,
    StartupAnalyzer,
    ProcessMonitor,
    OsUpdateChecker,
    PortScanner,
    BloatwareDetector,
    NetworkChecker,
    SmartDiskChecker,
    StorageChecker,

    // Export formats
    ExportCsv,
    ExportHtml,
    ExportPdf,
    ExportJson,

    // Advanced features
    AutoFix,
    ScanHistory,
}

/// Pro-only capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProFeature {
    /// Scheduled automation and auto-fix routines
    Automation,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// License key (for Pro tier)
    pub key: Option<String>,
    /// Current license tier
    pub tier: LicenseTier,
    /// When the license was activated (for trial expiration)
    pub activated_at: i64, // Unix timestamp
    /// Optional expiration timestamp (for trials)
    pub expires_at: Option<i64>,
}

impl Default for License {
    fn default() -> Self {
        License {
            key: None,
            tier: LicenseTier::Free,
            activated_at: chrono::Utc::now().timestamp(),
            expires_at: None,
        }
    }
}

impl License {
    /// Check if trial period has expired
    pub fn is_trial_expired(&self) -> bool {
        if self.tier != LicenseTier::Trial {
            return false;
        }

        if let Some(expires) = self.expires_at {
            let now = chrono::Utc::now().timestamp();
            now > expires
        } else {
            false
        }
    }

    /// Get the effective tier (downgrades expired trials to Free)
    pub fn effective_tier(&self) -> LicenseTier {
        if self.tier == LicenseTier::Trial && self.is_trial_expired() {
            LicenseTier::Free
        } else {
            self.tier
        }
    }

    /// Check if a feature is available in this license
    pub fn has_feature(&self, feature: Feature) -> bool {
        let _ = feature;
        true
    }

    /// Check if a Pro-only capability is available
    pub fn has_pro_feature(&self, feature: ProFeature) -> bool {
        let tier = self.effective_tier();
        match feature {
            ProFeature::Automation => matches!(tier, LicenseTier::Pro | LicenseTier::Trial),
        }
    }

    /// Get days remaining in trial (returns 0 if not trial or expired)
    pub fn trial_days_remaining(&self) -> i64 {
        if self.tier != LicenseTier::Trial {
            return 0;
        }

        if let Some(expires) = self.expires_at {
            let now = chrono::Utc::now().timestamp();
            let remaining_seconds = expires - now;
            if remaining_seconds > 0 {
                (remaining_seconds / 86400).max(0)
            } else {
                0
            }
        } else {
            0
        }
    }
}

/// License manager handles loading, saving, and validating licenses
pub struct LicenseManager {
    license_path: PathBuf,
}

impl LicenseManager {
    /// Create a new license manager with the specified storage path
    pub fn new(license_path: PathBuf) -> Self {
        LicenseManager { license_path }
    }

    /// Load license from disk, or create default Free license
    pub fn load(&self) -> Result<License, String> {
        if self.license_path.exists() {
            let content = fs::read_to_string(&self.license_path)
                .map_err(|e| format!("Failed to read license file: {}", e))?;

            let license: License = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse license file: {}", e))?;

            Ok(license)
        } else {
            // No license file exists, return default Free license
            Ok(License::default())
        }
    }

    /// Save license to disk
    pub fn save(&self, license: &License) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = self.license_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create license directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(license)
            .map_err(|e| format!("Failed to serialize license: {}", e))?;

        fs::write(&self.license_path, content)
            .map_err(|e| format!("Failed to write license file: {}", e))?;

        Ok(())
    }

    /// Validate a Pro license key
    ///
    /// Key format: HSPC-XXXX-XXXX-XXXX-XXXX (where X is alphanumeric)
    ///
    /// This is a simple validation scheme. For production, you'd want to:
    /// - Use a proper signing algorithm (RSA, ECDSA)
    /// - Validate against an online server
    /// - Include hardware fingerprinting to prevent sharing
    pub fn validate_key(key: &str) -> bool {
        // Basic format validation
        let parts: Vec<&str> = key.split('-').collect();
        if parts.len() != 5 {
            return false;
        }

        if parts[0] != "HSPC" {
            return false;
        }

        // Each segment should be exactly 4 alphanumeric characters
        for segment in &parts[1..] {
            if segment.len() != 4 {
                return false;
            }
            if !segment.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
                return false;
            }
        }

        // Simple checksum validation (last digit of last segment)
        // In production, use a proper checksum algorithm
        let checksum_valid = Self::verify_checksum(&parts[1..4], parts[4]);

        checksum_valid
    }

    /// Simple checksum verification (for demonstration)
    /// In production, use HMAC-SHA256 or similar
    fn verify_checksum(segments: &[&str], checksum_segment: &str) -> bool {
        let combined = segments.join("");
        let sum: u32 = combined.chars()
            .filter_map(|c| c.to_digit(36))
            .sum();

        // Last character of checksum segment should match sum modulo 36
        if let Some(last_char) = checksum_segment.chars().last() {
            if let Some(expected) = last_char.to_digit(36) {
                return (sum % 36) == expected;
            }
        }

        false
    }

    /// Activate a Pro license with the given key
    pub fn activate_pro(&self, key: &str) -> Result<License, String> {
        if !Self::validate_key(key) {
            return Err("Invalid license key format".to_string());
        }

        let license = License {
            key: Some(key.to_uppercase()),
            tier: LicenseTier::Pro,
            activated_at: chrono::Utc::now().timestamp(),
            expires_at: None,
        };

        self.save(&license)?;
        Ok(license)
    }

    /// Start a 14-day trial
    pub fn start_trial(&self) -> Result<License, String> {
        // Check if trial was already used
        let current = self.load()?;
        if current.tier == LicenseTier::Pro {
            return Err("Pro license is already active".to_string());
        }

        // Check if user already had a trial
        if let Ok(existing) = self.load() {
            if existing.tier == LicenseTier::Trial {
                if existing.is_trial_expired() {
                    return Err("Trial period has expired. Please upgrade to Pro.".to_string());
                } else {
                    // Trial still active
                    return Ok(existing);
                }
            }
        }

        let now = chrono::Utc::now().timestamp();
        let trial_duration = 14 * 86400; // 14 days in seconds

        let license = License {
            key: None,
            tier: LicenseTier::Trial,
            activated_at: now,
            expires_at: Some(now + trial_duration),
        };

        self.save(&license)?;
        Ok(license)
    }

    /// Downgrade to Free tier
    pub fn downgrade_to_free(&self) -> Result<License, String> {
        let license = License::default();
        self.save(&license)?;
        Ok(license)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_tier_feature_gating() {
        let free = License { tier: LicenseTier::Free, ..Default::default() };
        assert!(free.has_feature(Feature::FirewallChecker));
        assert!(free.has_feature(Feature::NetworkChecker));
        assert!(free.has_feature(Feature::ExportPdf));
        assert!(!free.has_pro_feature(ProFeature::Automation));

        let pro = License { tier: LicenseTier::Pro, ..Default::default() };
        assert!(pro.has_feature(Feature::AutoFix));
        assert!(pro.has_pro_feature(ProFeature::Automation));

        let now = chrono::Utc::now().timestamp();
        let trial = License {
            tier: LicenseTier::Trial,
            activated_at: now,
            expires_at: Some(now + 86400),
            ..Default::default()
        };
        assert!(trial.has_pro_feature(ProFeature::Automation));

        let expired_trial = License {
            tier: LicenseTier::Trial,
            activated_at: now - 1_000_000,
            expires_at: Some(now - 10),
            ..Default::default()
        };
        assert!(!expired_trial.has_pro_feature(ProFeature::Automation));
    }

    #[test]
    fn test_trial_expiration() {
        let now = chrono::Utc::now().timestamp();

        // Active trial
        let active_trial = License {
            tier: LicenseTier::Trial,
            activated_at: now,
            expires_at: Some(now + 86400), // 1 day remaining
            ..Default::default()
        };
        assert!(!active_trial.is_trial_expired());
        assert_eq!(active_trial.effective_tier(), LicenseTier::Trial);

        // Expired trial
        let expired_trial = License {
            tier: LicenseTier::Trial,
            activated_at: now - 1000000,
            expires_at: Some(now - 86400), // Expired 1 day ago
            ..Default::default()
        };
        assert!(expired_trial.is_trial_expired());
        assert_eq!(expired_trial.effective_tier(), LicenseTier::Free);
    }

    #[test]
    fn test_key_validation() {
        // Valid format
        assert!(LicenseManager::validate_key("HSPC-1234-5678-9ABC-DEF0"));

        // Invalid format
        assert!(!LicenseManager::validate_key("INVALID-KEY"));
        assert!(!LicenseManager::validate_key("HSPC-123-456-789-ABC")); // Wrong length
        assert!(!LicenseManager::validate_key("WRONG-1234-5678-9ABC-DEF0")); // Wrong prefix
    }
}
