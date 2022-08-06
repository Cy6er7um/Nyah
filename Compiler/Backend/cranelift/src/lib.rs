use std::collections::HashMap;
use cranelift::codegen::ir::GlobalValue;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::DataContext;
use nyah_il::global_data::GlobalDataRef;
use nyah_il::project::Project;

pub struct CraneliftJitProject {
    pub module: JITModule,
}

impl CraneliftJitProject {
    pub fn from_project<It, K>(_project: Project, symbols: It) -> Result<Self, String>
        where It: IntoIterator<Item=(K, *const u8)>, K: Into<String>, {
        let mut builder = JITBuilder::new(
            cranelift_module::default_libcall_names()
        ).map_err(|error| error.to_string())?;

        builder.hotswap(true);
        builder.symbols(symbols);

        let module = JITModule::new(builder);

        /*
        let mut data_context = DataContext::new();
        let mut global_data_id_map: HashMap<GlobalDataRef, GlobalValue> = HashMap::new();
        for (global_data_ref, global_data) in project.global_data_map {
            data_context.define_zeroinit(1);
            data_context
            global_data_id_map[&global_data_ref];
        }
        */

        Ok(CraneliftJitProject {
            module,
        })
    }
}
