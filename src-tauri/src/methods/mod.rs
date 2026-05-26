// Methods module - contains all renaming method implementations
// Organized by priority: P0 (core), P1 (extended)

pub mod p0_methods;
pub mod p1_methods;

use anyhow::Result;
use crate::method_engine::Method;
use crate::models::method_config::MethodConfig;

// Re-export all method types for convenient access
pub use p0_methods::{ReplaceMethod, AddMethod, RemoveMethod, NewCaseMethod};
pub use p1_methods::{ListMethod, MoveMethod, TrimMethod, RenumberMethod, TimestampMethod};

/// Factory function to create a Method instance from configuration
/// 
/// This is the main entry point for converting serialized method configs
/// (from frontend JSON) into executable Method objects for the pipeline.
/// 
/// # Arguments
/// * `config` - The deserialized method configuration
/// 
/// # Returns
/// A boxed Method trait object ready to be added to a Pipeline,
/// or an error if the configuration is invalid.
pub fn create_method_from_config(config: &MethodConfig) -> Result<Box<dyn Method>> {
    match config {
        MethodConfig::Replace(cfg) => {
            Ok(Box::new(ReplaceMethod::new(cfg.clone())))
        }
        
        MethodConfig::Add(cfg) => {
            Ok(Box::new(AddMethod::new(cfg.clone())))
        }
        
        MethodConfig::Remove(cfg) => {
            Ok(Box::new(RemoveMethod::new(cfg.clone())))
        }
        
        MethodConfig::NewCase(cfg) => {
            Ok(Box::new(NewCaseMethod::new(cfg.clone())))
        }
        
        MethodConfig::NewName(cfg) => {
            // NewName uses template-based renaming via the tag system
            // For now, use a simple template replacement
            Ok(Box::new(p0_methods::NewNameMethodAdapter::new(cfg.clone())))
        }
        
        MethodConfig::List(cfg) => {
            Ok(Box::new(ListMethod::new(cfg.clone())))
        }
        
        MethodConfig::Move(cfg) => {
            Ok(Box::new(MoveMethod::new(cfg.clone())))
        }
        
        MethodConfig::Trim(cfg) => {
            Ok(Box::new(TrimMethod::new(cfg.clone())))
        }
        
        MethodConfig::Renumber(cfg) => {
            Ok(Box::new(RenumberMethod::new(cfg.clone())))
        }
        
        MethodConfig::Timestamp(cfg) => {
            Ok(Box::new(TimestampMethod::new(cfg.clone())))
        }
    }
}
