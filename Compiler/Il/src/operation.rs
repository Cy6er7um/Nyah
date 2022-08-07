use crate::function::FunctionRef;
use crate::global_data::GlobalDataRef;
use crate::r#type::TypeRef;
use crate::stack::StackRef;
use crate::static_char_array::StaticCARef;
use crate::value::ValueRef;
use crate::variable::VariableRef;

#[derive(Clone)]
pub enum Operation {
    ConstI8(ValueRef, i8),
    ConstU8(ValueRef, u8),
    ConstI16(ValueRef, i16),
    ConstU16(ValueRef, u16),
    ConstI32(ValueRef, i32),
    ConstU32(ValueRef, u32),
    ConstF32(ValueRef, f32),
    ConstF64(ValueRef, f64),
    ConstChar8(ValueRef, u8),
    ConstChar32(ValueRef, char),

    LabelDeclare(u32),
    LabelJump(u32),
    LabelJumpIfTrue(u32, ValueRef),

    ParamGet(ValueRef, u32),
    DynamicParamArrayPointerGet(ValueRef),
    DynamicParamLengthGet(ValueRef),
    DynamicKeywordParamNameArrayPointerGet(ValueRef),
    DynamicKeywordParamObjectArrayPointerGet(ValueRef),
    DynamicKeywordParamLengthGet(ValueRef),

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
    Eq(ValueRef, ValueRef),
    // TODO: More operations...

    FunctionCall {
        function: FunctionRef,
        params: Vec<ValueRef>,
        dynamic_params: Option<Vec<ValueRef>>,
        dynamic_keyword_params: Option<Vec<(String, ValueRef)>>,
        r#return: Vec<ValueRef>,
    },
}
