use std::collections::HashMap;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataId, Module};
use nyah_il::global_data::GlobalDataRef;
use nyah_il::project::Project;

pub struct CraneliftJitProject {
    pub module: JITModule,
}

impl CraneliftJitProject {
    pub fn from_project<It, K>(project: Project, symbols: It) -> Result<Self, String>
        where It: IntoIterator<Item=(K, *const u8)>, K: Into<String>, {
        let mut builder = JITBuilder::new(
            cranelift_module::default_libcall_names()
        ).map_err(|error| error.to_string())?;

        builder.hotswap(true);
        builder.symbols(symbols);

        let mut module = JITModule::new(builder);

        let mut global_data_id_map: HashMap<GlobalDataRef, DataId> = HashMap::new();
        for (global_data_ref, global_data) in &project.global_data_map {
            match &global_data.name {
                Some(name) => {
                    let data_id = module.declare_data(
                        name.as_str(),
                        cranelift_module::Linkage::Export,
                        true,
                        false,
                    ).map_err(|error| error.to_string())?;
                    global_data_id_map.insert(*global_data_ref, data_id);
                }
                None => {
                    let data_id = module.declare_anonymous_data(
                        true,
                        false,
                    ).map_err(|error| error.to_string())?;
                    global_data_id_map.insert(*global_data_ref, data_id);
                }
            }
        }

        Ok(CraneliftJitProject {
            module,
        })
    }
}
