use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    pub fn parse(s: &str) -> Option<Scale> {
        match s {
            "international" => Some(Scale::International),
            "national" => Some(Scale::National),
            "regional" => Some(Scale::Regional),
            "local" => Some(Scale::Local),
            _ => None,
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Scale::International => "international",
            Scale::National => "national",
            Scale::Regional => "regional",
            Scale::Local => "local",
        };
        f.write_str(name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLabel {
    Estimated,
    Cited,
    Validated,
    Provisional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Peak,
    Average,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: EvidenceLabel,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: String,
    pub kind: String,
    pub scale: Option<Scale>,
    pub market: String,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
}

#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("corpus entry is missing a required id")]
    MissingId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoldReason {
    UncitedQuantity(String),
    MissingScale,
}

impl CorpusEntry {
    pub fn validate(&self) -> Result<Vec<HoldReason>, CorpusError> {
        if self.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        let mut reasons = Vec::new();

        if self.scale.is_none() {
            reasons.push(HoldReason::MissingScale);
        }

        for quantity in &self.quantities {
            if quantity.source_id.is_none() {
                reasons.push(HoldReason::UncitedQuantity(quantity.unit.clone()));
            }
        }

        Ok(reasons)
    }

    pub fn from_markdown(input: &str) -> Result<CorpusEntry, CorpusError> {
        let mut entry = CorpusEntry::default();

        let mut lines = input.lines();

        let mut in_frontmatter = false;
        for line in lines.by_ref() {
            if line.trim() == "---" {
                in_frontmatter = true;
                break;
            }
            if !line.trim().is_empty() {
                break;
            }
        }

        if in_frontmatter {
            for line in lines {
                if line.trim() == "---" {
                    break;
                }

                let mut parts = line.splitn(2, ':');
                let key = match parts.next() {
                    Some(k) => k.trim(),
                    None => continue,
                };
                let value = match parts.next() {
                    Some(v) => v.trim(),
                    None => continue,
                };

                match key {
                    "id" => entry.id = value.to_string(),
                    "kind" | "type" => entry.kind = value.to_string(),
                    "scale" => entry.scale = Scale::parse(value),
                    "market" => entry.market = value.to_string(),
                    "tier" if !value.is_empty() => entry.tier = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        if entry.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_rejected() {
        let entry = CorpusEntry::default();
        match entry.validate() {
            Err(CorpusError::MissingId) => {}
            other => panic!("expected MissingId, got {:?}", other),
        }
    }

    #[test]
    fn none_scale_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::MissingScale));
    }

    #[test]
    fn uncited_quantity_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::Local),
            quantities: vec![Quantity {
                value: 1.0,
                unit: "kg".to_string(),
                label: EvidenceLabel::Estimated,
                source_id: None,
            }],
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::UncitedQuantity("kg".to_string())));
    }

    #[test]
    fn evidence_label_unchanged_after_validate() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::Local),
            quantities: vec![Quantity {
                value: 1.0,
                unit: "kg".to_string(),
                label: EvidenceLabel::Validated,
                source_id: None,
            }],
            ..Default::default()
        };
        let _ = entry.validate().unwrap();
        assert_eq!(entry.quantities[0].label, EvidenceLabel::Validated);
    }

    #[test]
    fn from_markdown_parses_frontmatter() {
        let input = "---\nid: alpha\nkind: facility\nscale: national\nmarket: eu\n---\n";
        let entry = CorpusEntry::from_markdown(input).unwrap();
        assert_eq!(entry.id, "alpha");
        assert_eq!(entry.scale, Some(Scale::National));
    }
}
