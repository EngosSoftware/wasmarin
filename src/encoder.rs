#![allow(dead_code)]

use crate::Model;
use wasm_encoder::{Module, TypeSection};

/// The WebAssembly encoder.
pub struct Encoder {
  //
}

impl Default for Encoder {
  fn default() -> Self {
    Self::new()
  }
}

impl Encoder {
  pub fn new() -> Self {
    Self {}
  }

  pub fn encode(&self, _model: Model) -> Vec<u8> {
    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();
    let params = vec![wasm_encoder::ValType::I32, wasm_encoder::ValType::I32];
    let results = vec![wasm_encoder::ValType::I32];

    // for rec_group in model.rec_groups {
    //   let sub_types = rec_group
    //     .types()
    //     .map(|a| wasm_encoder::SubType {
    //       is_final: false,
    //       supertype_idx: None,
    //       composite_type: wasm_encoder::CompositeType { inner: (), shared: false },
    //     })
    //     .collect::<Vec<wasm_encoder::SubType>>();
    //   types.ty().rec(sub_types);
    // }

    types.ty().function(params, results);

    module.section(&types);

    // // Encode the function section.
    // let mut functions = FunctionSection::new();
    // let type_index = 0;
    // functions.function(type_index);
    // module.section(&functions);
    //
    // // Encode the export section.
    // let mut exports = ExportSection::new();
    // exports.export("f", ExportKind::Func, 0);
    // module.section(&exports);
    //
    // // Encode the code section.
    // let mut codes = CodeSection::new();
    // let locals = vec![];
    // let mut f = Function::new(locals);
    // f.instructions().local_get(0).local_get(1).i32_add().end();
    // codes.function(&f);
    // module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    module.finish()
  }
}

// fn map_sub_type(sub_type: wasmparser::SubType) -> wasm_encoder::SubType {
//   wasm_encoder::SubType{
//     is_final: sub_type.is_final,
//     supertype_idx: None,
//     composite_type: wasm_encoder::CompositeType { inner: wasm_encoder::CompositeInnerType::Array(wasm_encoder::ArrayType(wasm_encoder::FieldType{ element_type: StorageType::I8, mutable: false })), shared: false },
//   }
// }

// fn map_composite_type(composite_type: wasmparser::CompositeType) -> wasm_encoder::CompositeType {
//   wasm_encoder::CompositeType {
//     inner: map_composite_inner_type(),
//     shared: composite_type.shared,
//   }
// }

// fn map_composite_inner_type(composite_inner_type: wasmparser::CompositeInnerType) -> wasm_encoder::CompositeInnerType {
//   match composite_inner_type {
//     wasmparser::CompositeInnerType::Func(func_type) => wasm_encoder::CompositeInnerType::Func(map_func_type(func_type)),
//   wasmparser::CompositeInnerType::Array(array_type) => wasm_encoder::CompositeInnerType::Array(map_array_type(array_type)),
//   wasmparser::CompositeInnerType::Struct(_) => {}
//   wasmparser::CompositeInnerType::Cont(_) => {}
//   }
// }

fn map_field_type(field_type: wasmparser::FieldType) -> wasm_encoder::FieldType {
  wasm_encoder::FieldType {
    element_type: map_storage_type(field_type.element_type),
    mutable: field_type.mutable,
  }
}

fn map_storage_type(storage_type: wasmparser::StorageType) -> wasm_encoder::StorageType {
  match storage_type {
    wasmparser::StorageType::I8 => wasm_encoder::StorageType::I8,
    wasmparser::StorageType::I16 => wasm_encoder::StorageType::I16,
    wasmparser::StorageType::Val(val_type) => wasm_encoder::StorageType::Val(map_val_type(val_type)),
  }
}

fn map_func_type(func_type: wasmparser::FuncType) -> wasm_encoder::FuncType {
  wasm_encoder::FuncType::new(func_type.params().iter().map(|a| map_val_type(*a)), func_type.results().iter().map(|a| map_val_type(*a)))
}

fn map_array_type(array_type: wasmparser::ArrayType) -> wasm_encoder::ArrayType {
  wasm_encoder::ArrayType(map_field_type(array_type.0))
}

// fn map_struct_type(struct_type: wasmparser::StructType) -> wasm_encoder::StructType{
//   wasm_encoder::StructType{ fields: Box::new(struct_type.fields.iter().map(|a| map_field_type(*a)).collect()) }
// }

fn map_val_type(val_type: wasmparser::ValType) -> wasm_encoder::ValType {
  match val_type {
    wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
    wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
    wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
    wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
    wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
    wasmparser::ValType::Ref(ref_type) => wasm_encoder::ValType::Ref(map_ref_type(ref_type)),
  }
}

fn map_ref_type(ref_type: wasmparser::RefType) -> wasm_encoder::RefType {
  wasm_encoder::RefType {
    nullable: ref_type.is_nullable(),
    heap_type: map_heap_type(ref_type.heap_type()),
  }
}

fn map_heap_type(heap_type: wasmparser::HeapType) -> wasm_encoder::HeapType {
  match heap_type {
    wasmparser::HeapType::Abstract { shared, ty } => wasm_encoder::HeapType::Abstract {
      shared,
      ty: map_abstract_heap_type(ty),
    },
    wasmparser::HeapType::Concrete(_) => {
      //wasm_encoder::HeapType::Concrete(0)
      todo!()
    }
  }
}

fn map_abstract_heap_type(abstract_heap_type: wasmparser::AbstractHeapType) -> wasm_encoder::AbstractHeapType {
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

fn map_unpacked_index(unpacked_index: wasmparser::UnpackedIndex) -> u32 {
  match unpacked_index {
    wasmparser::UnpackedIndex::Module(a) => a,
    wasmparser::UnpackedIndex::RecGroup(a) => a,
    _ => todo!(),
  }
}
