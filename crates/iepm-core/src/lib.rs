use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    pub game: GameTarget,
    pub mods: Vec<RequestedMod>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameTarget {
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RequestedMod {
    Package(String),
    Selection {
        package: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<String>,
        #[serde(default)]
        components: Vec<String>,
    },
}

impl RequestedMod {
    pub fn package(&self) -> &str {
        match self {
            Self::Package(package) => package,
            Self::Selection { package, .. } => package,
        }
    }

    pub fn components(&self) -> &[String] {
        match self {
            Self::Package(_) => &[],
            Self::Selection { components, .. } => components,
        }
    }

    pub fn version(&self) -> Option<&str> {
        match self {
            Self::Package(_) => None,
            Self::Selection { version, .. } => version.as_deref(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackageRecord {
    pub schema: u32,
    pub package: String,
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub version: String,
    #[serde(default)]
    pub artifact: Option<Artifact>,
    pub compatibility: Compatibility,
    pub install: Install,
    pub provenance: Provenance,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artifact {
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Compatibility {
    pub games: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Install {
    pub phase: Phase,
    #[serde(default)]
    pub before: Vec<String>,
    #[serde(default)]
    pub after: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Phase {
    Preprocess,
    Bgee,
    EetImport,
    Eet,
    EetEnd,
    PostEetEnd,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    pub package: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub id: String,
    #[serde(default)]
    pub provides: Vec<Capability>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Capability {
    pub name: String,
    #[serde(default)]
    pub exclusive: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Provenance {
    Declared,
    Derived,
    Verified,
    Community,
    Unverified,
}

#[derive(Debug, Clone, Serialize)]
pub struct Lockfile {
    pub schema: u32,
    pub game: GameTarget,
    pub registry_revision: String,
    pub toolchain: Toolchain,
    pub packages: Vec<LockedPackage>,
    pub install_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Toolchain {
    pub iepm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weidu: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LockedPackage {
    pub package: String,
    pub version: String,
    pub phase: Phase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<Artifact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    pub provenance: Provenance,
}

pub type Registry = BTreeMap<String, PackageRecord>;
