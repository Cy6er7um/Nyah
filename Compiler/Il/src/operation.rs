use crate::function::FunctionRef;
use crate::global_data::GlobalDataRef;
use crate::r#type::TypeRef;
use crate::stack::StackRef;
use crate::static_char_array::StaticCARef;
use crate::value::ValueRef;
use crate::variable::VariableRef;

pub enum Operation {
    ConstI8(ValueRef, i32),
    ConstU8(ValueRef, i32),
    ConstI16(ValueRef, i32),
    ConstU16(ValueRef, i32),
    ConstU32(ValueRef, u32),
    ConstF32(ValueRef, f32),
    ConstF64(ValueRef, f64),
    ConstChar8(ValueRef, u8),
    ConstChar32(ValueRef, char),

    VariableSet(VariableRef, ValueRef),
    VariableGet(VariableRef, ValueRef),

    GlobalDataGetPointer(GlobalDataRef, ValueRef),
    StaticCAGetPointer(StaticCARef, ValueRef),

    Store(ValueRef, ValueRef),
    Read(ValueRef, ValueRef),

    StackAlloc(StackRef, TypeRef),
    StackStore(StackRef, ValueRef),
    StackRead(StackRef, ValueRef),

    Add(ValueRef, ValueRef),
    Sub(ValueRef, ValueRef),
    // TODO: More operations...

    FunctionCall {
        function: FunctionRef,
        params: Vec<ValueRef>,
        dynamic_params: Option<Vec<ValueRef>>,
        dynamic_keyword_params: Option<Vec<(String, ValueRef)>>,
        r#return: Vec<ValueRef>,
    },
}
