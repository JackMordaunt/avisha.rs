use crate::app::{Site, Tenant};
use crate::site_form;
use crate::tenant_form::Model as TenantFormModel;
use std::collections::HashMap;

pub trait Validate {
    type Model;

    fn validate(&self, m: &Self::Model) -> Result<(), HashMap<String, String>>;
}

#[derive(Clone, PartialEq)]
pub struct TenantValidator {
    pub tenants: HashMap<String, Tenant>,
}

impl Validate for TenantValidator {
    type Model = TenantFormModel;

    fn validate(&self, m: &Self::Model) -> Result<(), HashMap<String, String>> {
        let mut errors: HashMap<String, String> = HashMap::new();

        if m.name.is_empty() {
            errors.insert("name".into(), format!("must be non-zero"));
        }

        if self.tenants.contains_key(&m.name) {
            errors.insert("name".into(), format!("must be unique"));
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SiteValidator {
    pub sites: HashMap<String, Site>,
}

impl Validate for SiteValidator {
    type Model = site_form::Model;

    fn validate(&self, m: &Self::Model) -> Result<(), HashMap<String, String>> {
        let mut errors: HashMap<String, String> = HashMap::new();

        if m.number.is_empty() {
            errors.insert("number".into(), format!("must be non-zero"));
        }

        if self.sites.contains_key(&m.number) {
            errors.insert("number".into(), format!("must be unique"));
        }

        if let site_form::Kind::Other(k) = &m.kind {
            if k.is_empty() {
                errors.insert("kind".into(), format!("must be non-zero"));
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}
