use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trivy {
    #[serde(rename = "ArtifactName")]
    pub artifact_name: String,
    #[serde(rename = "Results")]
    pub results: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Vulnerabilities")]
    pub vulnerabilities: Option<Vec<Vulnerabilities>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerabilities {
    #[serde(rename = "VulnerabilityID")]
    pub vulnerability_id: Option<String>,
    #[serde(rename = "Severity")]
    pub severity: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "SeveritySource")]
    pub severity_source: Option<String>,
    #[serde(rename = "PkgName")]
    pub pkg_name: Option<String>,
    #[serde(rename = "InstalledVersion")]
    pub installed_version: Option<String>,
    #[serde(rename = "FixedVersion")]
    pub fixed_version: Option<String>,
}

pub fn trivy(image_name: &str) -> Trivy {
    // setup terminal
    let mut cmd = Command::new("trivy");
    let list = cmd.arg("image").arg(image_name)
        .arg("--format").arg("json")
        .output();

    let object: Trivy;
    match list {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(data) => {
                object = serde_json::from_str(&data.as_str()).unwrap();
            }
            Err(_) => unreachable!("No panic happens in this block"),
        }
        Err(_) => unreachable!("No panic happens in this block"),
    }
    object
}

