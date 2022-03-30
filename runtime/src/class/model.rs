use crate::class::field::NyahClassField;
use crate::class::getter::NyahClassGetter;
use crate::class::setter::NyahClassSetter;
use crate::function::reference::NyahFunctionReference;
use crate::interface::reference::NyahInterfaceReference;
use crate::util::indexed_map::NyahUtilIndexedMap;

pub struct NyahClassModel {
    pub name: String,

    pub fields: NyahUtilIndexedMap<String, NyahClassField>,

    pub getters: NyahUtilIndexedMap<String, NyahClassGetter>,
    pub setters: NyahUtilIndexedMap<String, NyahClassSetter>,

    pub methods: NyahUtilIndexedMap<String, NyahFunctionReference>,

    pub interfaces: NyahUtilIndexedMap<String, NyahInterfaceReference>,
}
