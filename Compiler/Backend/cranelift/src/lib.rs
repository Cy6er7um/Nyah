use std::collections::HashMap;
use cranelift::codegen::isa::CallConv;
use cranelift::prelude::{AbiParam, Signature};
use cranelift::prelude::types as cranelift_types;
use cranelift::prelude::Type as CraneliftType;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataId, FuncId, Module};
use nyah_il::function::{FunctionConv, FunctionRef};
use nyah_il::global_data::GlobalDataRef;
use nyah_il::project::Project;
use nyah_il::r#type::{Type, TypeBuiltin};

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
        let pointer_type = module.target_config().pointer_type();

        let mut global_data_id_map: HashMap<GlobalDataRef, DataId> = HashMap::new();
        for (global_data_ref, global_data) in &project.global_data_map {
            let data_id;
            match &global_data.name {
                Some(name) => {
                    data_id = module.declare_data(
                        name.as_str(),
                        cranelift_module::Linkage::Export,
                        true,
                        false,
                    );
                }
                None => {
                    data_id = module.declare_anonymous_data(
                        true,
                        false,
                    );
                }
            }
            global_data_id_map.insert(
                *global_data_ref,
                data_id.map_err(|error| error.to_string())?,
            );
        }

        let mut function_id_map: HashMap<FunctionRef, FuncId> = HashMap::new();
        for (function_ref, function) in &project.functions {
            let mut signature = Signature::new(
                match &function.conv {
                    FunctionConv::Fast => CallConv::Fast,
                    FunctionConv::C => module.target_config().default_call_conv,
                }
            );

            for t in &function.param_types {
                let il_type = project.types
                    .get(&t)
                    .ok_or(
                        String::from("Unknown type reference.")
                    )?;
                signature.params.push(
                    AbiParam::new(
                        il_type_to_cranelift_type(il_type, pointer_type)
                            .ok_or(
                                String::from("Illegal element count.")
                            )?
                    )
                );
            }

            if function.dynamic_param_type.is_some() {
                for _ in 0..2 {
                    signature.params.push(AbiParam::new(pointer_type));
                }
            }

            if function.dynamic_keyword_param_type.is_some() {
                for _ in 0..3 {
                    signature.params.push(AbiParam::new(pointer_type));
                }
            }

            for t in &function.return_types {
                let il_type = project.types
                    .get(&t)
                    .ok_or(
                        String::from("Unknown type reference.")
                    )?;
                signature.returns.push(
                    AbiParam::new(
                        il_type_to_cranelift_type(il_type, pointer_type)
                            .ok_or(
                                String::from("Illegal element count.")
                            )?
                    )
                );
            }

            let function_id;
            match &function.name {
                Some(name) => {
                    function_id = module.declare_function(
                        name.as_str(),
                        cranelift_module::Linkage::Export,
                        &signature,
                    );
                }
                None => {
                    function_id = module.declare_anonymous_function(
                        &signature,
                    );
                }
            }
            function_id_map.insert(
                *function_ref,
                function_id.map_err(|error| error.to_string())?,
            );
        }



        Ok(CraneliftJitProject {
            module,
        })
    }

    pub fn get_function_pointer() {}
}

pub fn il_type_to_cranelift_type(il_type: &Type, target_pointer_type: CraneliftType) -> Option<CraneliftType> {
    let element_type: CraneliftType = match &il_type.element_type {
        TypeBuiltin::I8 | TypeBuiltin::U8 | TypeBuiltin::C8 => cranelift_types::I8,
        TypeBuiltin::I16 | TypeBuiltin::U16 => cranelift_types::I16,
        TypeBuiltin::I32 | TypeBuiltin::U32 | TypeBuiltin::C32 => cranelift_types::I32,
        TypeBuiltin::I64 | TypeBuiltin::U64 => cranelift_types::I64,
        TypeBuiltin::F32 => cranelift_types::F32,
        TypeBuiltin::F64 => cranelift_types::F64,
        TypeBuiltin::B8 => cranelift_types::B8,
        TypeBuiltin::Pointer => target_pointer_type,
    };

    element_type.by(il_type.element_count)
}
