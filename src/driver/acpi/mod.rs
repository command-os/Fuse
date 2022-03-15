//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use acpi::tables::{RsdtType, SdtHeader};
use hashbrown::HashMap;
use log::info;

pub mod madt;

#[derive(Debug)]
pub struct Acpi {
    pub version: u8,
    pub tables: HashMap<&'static str, &'static SdtHeader>,
}

impl Acpi {
    pub fn new(rsdp: &'static acpi::tables::Rsdp) -> Self {
        let mut tables = HashMap::new();

        match rsdp.into_type() {
            RsdtType::Rsdt(rsdt) => {
                for ent in rsdt.iter() {
                    if ent.validate() {
                        tables.try_insert(ent.signature(), ent).unwrap();
                    } else {
                        info!("Invalid table: {:?}", ent);
                    }
                }
            }
            RsdtType::Xsdt(xsdt) => {
                for ent in xsdt.iter() {
                    if ent.validate() {
                        tables.try_insert(ent.signature(), ent).unwrap();
                    } else {
                        info!("Invalid table: {:?}", ent);
                    }
                }
            }
        }

        Self {
            version: rsdp.revision,
            tables,
        }
    }

    pub fn find<T>(&self, signature: &str) -> Option<&'static T> {
        self.tables
            .iter()
            .find(|(&a, _)| a == signature)
            .map(|(_, &v)| unsafe { (v as *const _ as *const T).as_ref().unwrap() })
    }
}