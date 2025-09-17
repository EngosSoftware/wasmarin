use wasmparser::types::TypeIdentifier;

pub fn map_memory_type(memory_type: &wasmparser::MemoryType) -> wasm_encoder::MemoryType {
  wasm_encoder::MemoryType {
    minimum: memory_type.initial,
    maximum: memory_type.maximum,
    memory64: memory_type.memory64,
    shared: memory_type.shared,
    page_size_log2: memory_type.page_size_log2,
  }
}

pub fn map_sub_type(sub_type: &wasmparser::SubType) -> wasm_encoder::SubType {
  wasm_encoder::SubType {
    is_final: sub_type.is_final,
    supertype_idx: sub_type.supertype_idx.map(map_packed_index),
    composite_type: map_composite_type(&sub_type.composite_type),
  }
}

pub fn map_composite_type(composite_type: &wasmparser::CompositeType) -> wasm_encoder::CompositeType {
  wasm_encoder::CompositeType {
    inner: map_composite_inner_type(&composite_type.inner),
    shared: composite_type.shared,
  }
}

pub fn map_composite_inner_type(composite_inner_type: &wasmparser::CompositeInnerType) -> wasm_encoder::CompositeInnerType {
  match composite_inner_type {
    wasmparser::CompositeInnerType::Func(func_type) => wasm_encoder::CompositeInnerType::Func(map_func_type(func_type)),
    wasmparser::CompositeInnerType::Array(array_type) => wasm_encoder::CompositeInnerType::Array(map_array_type(array_type)),
    wasmparser::CompositeInnerType::Struct(struct_type) => wasm_encoder::CompositeInnerType::Struct(map_struct_type(struct_type)),
    wasmparser::CompositeInnerType::Cont(cont_type) => wasm_encoder::CompositeInnerType::Cont(map_cont_type(cont_type)),
  }
}

pub fn map_field_type(field_type: &wasmparser::FieldType) -> wasm_encoder::FieldType {
  wasm_encoder::FieldType {
    element_type: map_storage_type(&field_type.element_type),
    mutable: field_type.mutable,
  }
}

pub fn map_storage_type(storage_type: &wasmparser::StorageType) -> wasm_encoder::StorageType {
  match storage_type {
    wasmparser::StorageType::I8 => wasm_encoder::StorageType::I8,
    wasmparser::StorageType::I16 => wasm_encoder::StorageType::I16,
    wasmparser::StorageType::Val(val_type) => wasm_encoder::StorageType::Val(map_val_type(val_type)),
  }
}

pub fn map_func_type(func_type: &wasmparser::FuncType) -> wasm_encoder::FuncType {
  wasm_encoder::FuncType::new(func_type.params().iter().map(map_val_type), func_type.results().iter().map(map_val_type))
}

pub fn map_array_type(array_type: &wasmparser::ArrayType) -> wasm_encoder::ArrayType {
  wasm_encoder::ArrayType(map_field_type(&array_type.0))
}

pub fn map_struct_type(struct_type: &wasmparser::StructType) -> wasm_encoder::StructType {
  let fields = struct_type.fields.iter().map(map_field_type).collect::<Vec<wasm_encoder::FieldType>>().into_boxed_slice();
  wasm_encoder::StructType { fields }
}

pub fn map_cont_type(cont_type: &wasmparser::ContType) -> wasm_encoder::ContType {
  wasm_encoder::ContType(map_packed_index(cont_type.0))
}

pub fn map_val_type(val_type: &wasmparser::ValType) -> wasm_encoder::ValType {
  match val_type {
    wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
    wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
    wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
    wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
    wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
    wasmparser::ValType::Ref(ref_type) => wasm_encoder::ValType::Ref(map_ref_type(ref_type)),
  }
}

pub fn map_ref_type(ref_type: &wasmparser::RefType) -> wasm_encoder::RefType {
  wasm_encoder::RefType {
    nullable: ref_type.is_nullable(),
    heap_type: map_heap_type(&ref_type.heap_type()),
  }
}

pub fn map_heap_type(heap_type: &wasmparser::HeapType) -> wasm_encoder::HeapType {
  match heap_type {
    wasmparser::HeapType::Abstract { shared, ty } => wasm_encoder::HeapType::Abstract {
      shared: *shared,
      ty: map_abstract_heap_type(ty),
    },
    wasmparser::HeapType::Concrete(unpacked_index) => wasm_encoder::HeapType::Concrete(map_unpacked_index(*unpacked_index)),
  }
}

pub fn map_abstract_heap_type(abstract_heap_type: &wasmparser::AbstractHeapType) -> wasm_encoder::AbstractHeapType {
  match abstract_heap_type {
    wasmparser::AbstractHeapType::Func => wasm_encoder::AbstractHeapType::Func,
    wasmparser::AbstractHeapType::Extern => wasm_encoder::AbstractHeapType::Extern,
    wasmparser::AbstractHeapType::Any => wasm_encoder::AbstractHeapType::Any,
    wasmparser::AbstractHeapType::None => wasm_encoder::AbstractHeapType::None,
    wasmparser::AbstractHeapType::NoExtern => wasm_encoder::AbstractHeapType::NoExtern,
    wasmparser::AbstractHeapType::NoFunc => wasm_encoder::AbstractHeapType::NoFunc,
    wasmparser::AbstractHeapType::Eq => wasm_encoder::AbstractHeapType::Eq,
    wasmparser::AbstractHeapType::Struct => wasm_encoder::AbstractHeapType::Struct,
    wasmparser::AbstractHeapType::Array => wasm_encoder::AbstractHeapType::Array,
    wasmparser::AbstractHeapType::I31 => wasm_encoder::AbstractHeapType::I31,
    wasmparser::AbstractHeapType::Exn => wasm_encoder::AbstractHeapType::Exn,
    wasmparser::AbstractHeapType::NoExn => wasm_encoder::AbstractHeapType::NoExn,
    wasmparser::AbstractHeapType::Cont => wasm_encoder::AbstractHeapType::Cont,
    wasmparser::AbstractHeapType::NoCont => wasm_encoder::AbstractHeapType::NoCont,
  }
}

pub fn map_packed_index(packed_index: wasmparser::PackedIndex) -> u32 {
  map_unpacked_index(packed_index.unpack())
}

pub fn map_unpacked_index(unpacked_index: wasmparser::UnpackedIndex) -> u32 {
  match unpacked_index {
    wasmparser::UnpackedIndex::Module(module_id) => module_id,
    wasmparser::UnpackedIndex::RecGroup(rec_group_id) => rec_group_id,
    wasmparser::UnpackedIndex::Id(core_type_id) => core_type_id.index() as u32,
  }
}
