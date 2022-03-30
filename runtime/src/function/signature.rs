use crate::class::reference::NyahClassReference;

pub struct NyahFunctionSignature {
    pub arg_types: Vec<NyahClassReference>,
    pub vararg_type: NyahClassReference,
    pub return_types: Vec<NyahClassReference>,
}
