use std::borrow::Cow;
use wasmparser::types::TypeIdentifier;

pub fn map_catches(mut catches: Vec<wasmparser::Catch>) -> Vec<wasm_encoder::Catch> {
  catches.drain(..).map(map_catch).collect()
}

pub fn map_catch(catch: wasmparser::Catch) -> wasm_encoder::Catch {
  match catch {
    wasmparser::Catch::One { tag, label } => wasm_encoder::Catch::One { tag, label },
    wasmparser::Catch::OneRef { tag, label } => wasm_encoder::Catch::OneRef { tag, label },
    wasmparser::Catch::All { label } => wasm_encoder::Catch::All { label },
    wasmparser::Catch::AllRef { label } => wasm_encoder::Catch::AllRef { label },
  }
}

pub fn map_ordering(ordering: wasmparser::Ordering) -> wasm_encoder::Ordering {
  match ordering {
    wasmparser::Ordering::AcqRel => wasm_encoder::Ordering::AcqRel,
    wasmparser::Ordering::SeqCst => wasm_encoder::Ordering::SeqCst,
  }
}

pub fn map_resume_table(mut resume_table: wasmparser::ResumeTable) -> Vec<wasm_encoder::Handle> {
  resume_table.handlers.drain(..).map(map_handle).collect()
}

pub fn map_handle(handle: wasmparser::Handle) -> wasm_encoder::Handle {
  match handle {
    wasmparser::Handle::OnLabel { tag, label } => wasm_encoder::Handle::OnLabel { tag, label },
    wasmparser::Handle::OnSwitch { tag } => wasm_encoder::Handle::OnSwitch { tag },
  }
}

pub fn map_global_type(global_type: wasmparser::GlobalType) -> wasm_encoder::GlobalType {
  wasm_encoder::GlobalType {
    val_type: map_val_type(global_type.content_type),
    mutable: global_type.mutable,
    shared: global_type.shared,
  }
}

pub fn map_const_expr(const_expr: wasmparser::ConstExpr) -> wasm_encoder::ConstExpr {
  let reader = const_expr.get_operators_reader();
  let mut instructions = vec![];
  for item in reader {
    let operator = item.unwrap();
    instructions.push(map_operator(operator));
  }
  if let Some(instruction) = instructions.last() {
    if matches!(instruction, wasm_encoder::Instruction::End) {
      instructions.remove(instructions.len() - 1);
    }
  }
  wasm_encoder::ConstExpr::extended(instructions)
}

pub fn map_export_kind(external_kind: wasmparser::ExternalKind) -> wasm_encoder::ExportKind {
  match external_kind {
    wasmparser::ExternalKind::Func => wasm_encoder::ExportKind::Func,
    wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
    wasmparser::ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
    wasmparser::ExternalKind::Global => wasm_encoder::ExportKind::Global,
    wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
  }
}

pub fn map_memory_type(memory_type: wasmparser::MemoryType) -> wasm_encoder::MemoryType {
  wasm_encoder::MemoryType {
    minimum: memory_type.initial,
    maximum: memory_type.maximum,
    memory64: memory_type.memory64,
    shared: memory_type.shared,
    page_size_log2: memory_type.page_size_log2,
  }
}

pub fn map_sub_type(sub_type: wasmparser::SubType) -> wasm_encoder::SubType {
  wasm_encoder::SubType {
    is_final: sub_type.is_final,
    supertype_idx: sub_type.supertype_idx.map(map_packed_index),
    composite_type: map_composite_type(sub_type.composite_type),
  }
}

pub fn map_composite_type(composite_type: wasmparser::CompositeType) -> wasm_encoder::CompositeType {
  wasm_encoder::CompositeType {
    inner: map_composite_inner_type(composite_type.inner),
    shared: composite_type.shared,
  }
}

pub fn map_composite_inner_type(composite_inner_type: wasmparser::CompositeInnerType) -> wasm_encoder::CompositeInnerType {
  match composite_inner_type {
    wasmparser::CompositeInnerType::Func(func_type) => wasm_encoder::CompositeInnerType::Func(map_func_type(func_type)),
    wasmparser::CompositeInnerType::Array(array_type) => wasm_encoder::CompositeInnerType::Array(map_array_type(array_type)),
    wasmparser::CompositeInnerType::Struct(struct_type) => wasm_encoder::CompositeInnerType::Struct(map_struct_type(struct_type)),
    wasmparser::CompositeInnerType::Cont(cont_type) => wasm_encoder::CompositeInnerType::Cont(map_cont_type(cont_type)),
  }
}

pub fn map_field_type(field_type: &wasmparser::FieldType) -> wasm_encoder::FieldType {
  wasm_encoder::FieldType {
    element_type: map_storage_type(field_type.element_type),
    mutable: field_type.mutable,
  }
}

pub fn map_storage_type(storage_type: wasmparser::StorageType) -> wasm_encoder::StorageType {
  match storage_type {
    wasmparser::StorageType::I8 => wasm_encoder::StorageType::I8,
    wasmparser::StorageType::I16 => wasm_encoder::StorageType::I16,
    wasmparser::StorageType::Val(val_type) => wasm_encoder::StorageType::Val(map_val_type(val_type)),
  }
}

pub fn map_func_type(func_type: wasmparser::FuncType) -> wasm_encoder::FuncType {
  wasm_encoder::FuncType::new(func_type.params().iter().cloned().map(map_val_type), func_type.results().iter().cloned().map(map_val_type))
}

pub fn map_array_type(array_type: wasmparser::ArrayType) -> wasm_encoder::ArrayType {
  wasm_encoder::ArrayType(map_field_type(&array_type.0))
}

pub fn map_struct_type(struct_type: wasmparser::StructType) -> wasm_encoder::StructType {
  let fields = struct_type.fields.iter().map(map_field_type).collect::<Vec<wasm_encoder::FieldType>>().into_boxed_slice();
  wasm_encoder::StructType { fields }
}

pub fn map_cont_type(cont_type: wasmparser::ContType) -> wasm_encoder::ContType {
  wasm_encoder::ContType(map_packed_index(cont_type.0))
}

pub fn map_val_type(val_type: wasmparser::ValType) -> wasm_encoder::ValType {
  match val_type {
    wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
    wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
    wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
    wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
    wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
    wasmparser::ValType::Ref(ref_type) => wasm_encoder::ValType::Ref(map_ref_type(ref_type)),
  }
}

pub fn map_val_types(mut val_types: Vec<wasmparser::ValType>) -> Vec<wasm_encoder::ValType> {
  val_types.drain(..).map(map_val_type).collect()
}

pub fn map_ref_type(ref_type: wasmparser::RefType) -> wasm_encoder::RefType {
  wasm_encoder::RefType {
    nullable: ref_type.is_nullable(),
    heap_type: map_heap_type(ref_type.heap_type()),
  }
}

pub fn map_heap_type(heap_type: wasmparser::HeapType) -> wasm_encoder::HeapType {
  match heap_type {
    wasmparser::HeapType::Abstract { shared, ty } => wasm_encoder::HeapType::Abstract {
      shared,
      ty: map_abstract_heap_type(ty),
    },
    wasmparser::HeapType::Concrete(unpacked_index) => wasm_encoder::HeapType::Concrete(map_unpacked_index(unpacked_index)),
  }
}

pub fn map_abstract_heap_type(abstract_heap_type: wasmparser::AbstractHeapType) -> wasm_encoder::AbstractHeapType {
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

pub fn map_block_type(block_type: wasmparser::BlockType) -> wasm_encoder::BlockType {
  match block_type {
    wasmparser::BlockType::Empty => wasm_encoder::BlockType::Empty,
    wasmparser::BlockType::Type(val_type) => wasm_encoder::BlockType::Result(map_val_type(val_type)),
    wasmparser::BlockType::FuncType(index) => wasm_encoder::BlockType::FunctionType(index),
  }
}

pub fn map_ieee32(ieee32: wasmparser::Ieee32) -> wasm_encoder::Ieee32 {
  wasm_encoder::Ieee32::new(ieee32.bits())
}

pub fn map_ieee64(ieee64: wasmparser::Ieee64) -> wasm_encoder::Ieee64 {
  wasm_encoder::Ieee64::new(ieee64.bits())
}

pub fn map_mem_arg(mem_arg: wasmparser::MemArg) -> wasm_encoder::MemArg {
  wasm_encoder::MemArg {
    offset: mem_arg.offset,
    align: mem_arg.align as u32,
    memory_index: mem_arg.memory,
  }
}

#[rustfmt::skip]
pub fn map_operator<'a>(operator: wasmparser::Operator) -> wasm_encoder::Instruction<'a> {
  match operator {
    wasmparser::Operator::Unreachable => wasm_encoder::Instruction::Unreachable,
    wasmparser::Operator::Nop => wasm_encoder::Instruction::Nop,
    wasmparser::Operator::Block { blockty } => wasm_encoder::Instruction::Block(map_block_type(blockty)),
    wasmparser::Operator::Loop { blockty } => wasm_encoder::Instruction::Loop(map_block_type(blockty)),
    wasmparser::Operator::If { blockty } => wasm_encoder::Instruction::If(map_block_type(blockty)),
    wasmparser::Operator::Else => wasm_encoder::Instruction::Else,
    wasmparser::Operator::End => wasm_encoder::Instruction::End,
    wasmparser::Operator::Br { relative_depth } => wasm_encoder::Instruction::Br(relative_depth),
    wasmparser::Operator::BrIf { relative_depth } => wasm_encoder::Instruction::BrIf(relative_depth),
    wasmparser::Operator::BrTable { targets } => wasm_encoder::Instruction::BrTable(Cow::Owned(targets.targets().collect::<Result<Vec<_>, _>>().unwrap()), targets.default()),
    wasmparser::Operator::Return => wasm_encoder::Instruction::Return,
    wasmparser::Operator::Call { function_index } => wasm_encoder::Instruction::Call(function_index),
    wasmparser::Operator::CallIndirect { type_index, table_index } => wasm_encoder::Instruction::CallIndirect { type_index, table_index },
    wasmparser::Operator::Drop => wasm_encoder::Instruction::Drop,
    wasmparser::Operator::Select => wasm_encoder::Instruction::Select,
    wasmparser::Operator::LocalGet { local_index } => wasm_encoder::Instruction::LocalGet(local_index),
    wasmparser::Operator::LocalSet { local_index } => wasm_encoder::Instruction::LocalSet(local_index),
    wasmparser::Operator::LocalTee { local_index } => wasm_encoder::Instruction::LocalTee(local_index),
    wasmparser::Operator::GlobalGet { global_index } => wasm_encoder::Instruction::GlobalGet(global_index),
    wasmparser::Operator::GlobalSet { global_index } => wasm_encoder::Instruction::GlobalSet(global_index),
    wasmparser::Operator::I32Load { memarg } => wasm_encoder::Instruction::I32Load(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load { memarg } => wasm_encoder::Instruction::I64Load(map_mem_arg(memarg)),
    wasmparser::Operator::F32Load { memarg } => wasm_encoder::Instruction::F32Load(map_mem_arg(memarg)),
    wasmparser::Operator::F64Load { memarg } => wasm_encoder::Instruction::F64Load(map_mem_arg(memarg)),
    wasmparser::Operator::I32Load8S { memarg } => wasm_encoder::Instruction::I32Load8S(map_mem_arg(memarg)),
    wasmparser::Operator::I32Load8U { memarg } => wasm_encoder::Instruction::I32Load8U(map_mem_arg(memarg)),
    wasmparser::Operator::I32Load16S { memarg } => wasm_encoder::Instruction::I32Load16S(map_mem_arg(memarg)),
    wasmparser::Operator::I32Load16U { memarg } => wasm_encoder::Instruction::I32Load16U(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load8S { memarg } => wasm_encoder::Instruction::I64Load8S(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load8U { memarg } => wasm_encoder::Instruction::I64Load8U(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load16S { memarg } => wasm_encoder::Instruction::I64Load16S(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load16U { memarg } => wasm_encoder::Instruction::I64Load16U(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load32S { memarg } => wasm_encoder::Instruction::I64Load32S(map_mem_arg(memarg)),
    wasmparser::Operator::I64Load32U { memarg } => wasm_encoder::Instruction::I64Load32U(map_mem_arg(memarg)),
    wasmparser::Operator::I32Store { memarg } => wasm_encoder::Instruction::I32Store(map_mem_arg(memarg)),
    wasmparser::Operator::I64Store { memarg } => wasm_encoder::Instruction::I64Store(map_mem_arg(memarg)),
    wasmparser::Operator::F32Store { memarg } => wasm_encoder::Instruction::F32Store(map_mem_arg(memarg)),
    wasmparser::Operator::F64Store { memarg } => wasm_encoder::Instruction::F64Store(map_mem_arg(memarg)),
    wasmparser::Operator::I32Store8 { memarg } => wasm_encoder::Instruction::I32Store8(map_mem_arg(memarg)),
    wasmparser::Operator::I32Store16 { memarg } => wasm_encoder::Instruction::I32Store16(map_mem_arg(memarg)),
    wasmparser::Operator::I64Store8 { memarg } => wasm_encoder::Instruction::I64Store8(map_mem_arg(memarg)),
    wasmparser::Operator::I64Store16 { memarg } => wasm_encoder::Instruction::I64Store16(map_mem_arg(memarg)),
    wasmparser::Operator::I64Store32 { memarg } => wasm_encoder::Instruction::I64Store32(map_mem_arg(memarg)),
    wasmparser::Operator::MemorySize { mem } => wasm_encoder::Instruction::MemorySize(mem),
    wasmparser::Operator::MemoryGrow { mem } => wasm_encoder::Instruction::MemoryGrow(mem),
    wasmparser::Operator::I32Const { value } => wasm_encoder::Instruction::I32Const(value),
    wasmparser::Operator::I64Const { value } => wasm_encoder::Instruction::I64Const(value),
    wasmparser::Operator::F32Const { value } => wasm_encoder::Instruction::F32Const(map_ieee32(value)),
    wasmparser::Operator::F64Const { value } => wasm_encoder::Instruction::F64Const(map_ieee64(value)),
    wasmparser::Operator::I32Eqz => wasm_encoder::Instruction::I32Eqz,
    wasmparser::Operator::I32Eq => wasm_encoder::Instruction::I32Eq,
    wasmparser::Operator::I32Ne => wasm_encoder::Instruction::I32Ne,
    wasmparser::Operator::I32LtS => wasm_encoder::Instruction::I32LtS,
    wasmparser::Operator::I32LtU => wasm_encoder::Instruction::I32LtU,
    wasmparser::Operator::I32GtS => wasm_encoder::Instruction::I32GtS,
    wasmparser::Operator::I32GtU => wasm_encoder::Instruction::I32GtU,
    wasmparser::Operator::I32LeS => wasm_encoder::Instruction::I32LeS,
    wasmparser::Operator::I32LeU => wasm_encoder::Instruction::I32LeU,
    wasmparser::Operator::I32GeS => wasm_encoder::Instruction::I32GeS,
    wasmparser::Operator::I32GeU => wasm_encoder::Instruction::I32GeU,
    wasmparser::Operator::I64Eqz => wasm_encoder::Instruction::I64Eqz,
    wasmparser::Operator::I64Eq => wasm_encoder::Instruction::I64Eq,
    wasmparser::Operator::I64Ne => wasm_encoder::Instruction::I64Ne,
    wasmparser::Operator::I64LtS => wasm_encoder::Instruction::I64LtS,
    wasmparser::Operator::I64LtU => wasm_encoder::Instruction::I64LtU,
    wasmparser::Operator::I64GtS => wasm_encoder::Instruction::I64GtS,
    wasmparser::Operator::I64GtU => wasm_encoder::Instruction::I64GtU,
    wasmparser::Operator::I64LeS => wasm_encoder::Instruction::I64LeS,
    wasmparser::Operator::I64LeU => wasm_encoder::Instruction::I64LeU,
    wasmparser::Operator::I64GeS => wasm_encoder::Instruction::I64GeS,
    wasmparser::Operator::I64GeU => wasm_encoder::Instruction::I64GeU,
    wasmparser::Operator::F32Eq => wasm_encoder::Instruction::F32Eq,
    wasmparser::Operator::F32Ne => wasm_encoder::Instruction::F32Ne,
    wasmparser::Operator::F32Lt => wasm_encoder::Instruction::F32Lt,
    wasmparser::Operator::F32Gt => wasm_encoder::Instruction::F32Gt,
    wasmparser::Operator::F32Le => wasm_encoder::Instruction::F32Le,
    wasmparser::Operator::F32Ge => wasm_encoder::Instruction::F32Ge,
    wasmparser::Operator::F64Eq => wasm_encoder::Instruction::F64Eq,
    wasmparser::Operator::F64Ne => wasm_encoder::Instruction::F64Ne,
    wasmparser::Operator::F64Lt => wasm_encoder::Instruction::F64Lt,
    wasmparser::Operator::F64Gt => wasm_encoder::Instruction::F64Gt,
    wasmparser::Operator::F64Le => wasm_encoder::Instruction::F64Le,
    wasmparser::Operator::F64Ge => wasm_encoder::Instruction::F64Ge,
    wasmparser::Operator::I32Clz => wasm_encoder::Instruction::I32Clz,
    wasmparser::Operator::I32Ctz => wasm_encoder::Instruction::I32Ctz,
    wasmparser::Operator::I32Popcnt => wasm_encoder::Instruction::I32Popcnt,
    wasmparser::Operator::I32Add => wasm_encoder::Instruction::I32Add,
    wasmparser::Operator::I32Sub => wasm_encoder::Instruction::I32Sub,
    wasmparser::Operator::I32Mul => wasm_encoder::Instruction::I32Mul,
    wasmparser::Operator::I32DivS => wasm_encoder::Instruction::I32DivS,
    wasmparser::Operator::I32DivU => wasm_encoder::Instruction::I32DivU,
    wasmparser::Operator::I32RemS => wasm_encoder::Instruction::I32RemS,
    wasmparser::Operator::I32RemU => wasm_encoder::Instruction::I32RemU,
    wasmparser::Operator::I32And => wasm_encoder::Instruction::I32And,
    wasmparser::Operator::I32Or => wasm_encoder::Instruction::I32Or,
    wasmparser::Operator::I32Xor => wasm_encoder::Instruction::I32Xor,
    wasmparser::Operator::I32Shl => wasm_encoder::Instruction::I32Shl,
    wasmparser::Operator::I32ShrS => wasm_encoder::Instruction::I32ShrS,
    wasmparser::Operator::I32ShrU => wasm_encoder::Instruction::I32ShrU,
    wasmparser::Operator::I32Rotl => wasm_encoder::Instruction::I32Rotl,
    wasmparser::Operator::I32Rotr => wasm_encoder::Instruction::I32Rotr,
    wasmparser::Operator::I64Clz => wasm_encoder::Instruction::I64Clz,
    wasmparser::Operator::I64Ctz => wasm_encoder::Instruction::I64Ctz,
    wasmparser::Operator::I64Popcnt => wasm_encoder::Instruction::I64Popcnt,
    wasmparser::Operator::I64Add => wasm_encoder::Instruction::I64Add,
    wasmparser::Operator::I64Sub => wasm_encoder::Instruction::I64Sub,
    wasmparser::Operator::I64Mul => wasm_encoder::Instruction::I64Mul,
    wasmparser::Operator::I64DivS => wasm_encoder::Instruction::I64DivS,
    wasmparser::Operator::I64DivU => wasm_encoder::Instruction::I64DivU,
    wasmparser::Operator::I64RemS => wasm_encoder::Instruction::I64RemS,
    wasmparser::Operator::I64RemU => wasm_encoder::Instruction::I64RemU,
    wasmparser::Operator::I64And => wasm_encoder::Instruction::I64And,
    wasmparser::Operator::I64Or => wasm_encoder::Instruction::I64Or,
    wasmparser::Operator::I64Xor => wasm_encoder::Instruction::I64Xor,
    wasmparser::Operator::I64Shl => wasm_encoder::Instruction::I64Shl,
    wasmparser::Operator::I64ShrS => wasm_encoder::Instruction::I64ShrS,
    wasmparser::Operator::I64ShrU => wasm_encoder::Instruction::I64ShrU,
    wasmparser::Operator::I64Rotl => wasm_encoder::Instruction::I64Rotl,
    wasmparser::Operator::I64Rotr => wasm_encoder::Instruction::I64Rotr,
    wasmparser::Operator::F32Abs => wasm_encoder::Instruction::F32Abs,
    wasmparser::Operator::F32Neg => wasm_encoder::Instruction::F32Neg,
    wasmparser::Operator::F32Ceil => wasm_encoder::Instruction::F32Ceil,
    wasmparser::Operator::F32Floor => wasm_encoder::Instruction::F32Floor,
    wasmparser::Operator::F32Trunc => wasm_encoder::Instruction::F32Trunc,
    wasmparser::Operator::F32Nearest => wasm_encoder::Instruction::F32Nearest,
    wasmparser::Operator::F32Sqrt => wasm_encoder::Instruction::F32Sqrt,
    wasmparser::Operator::F32Add => wasm_encoder::Instruction::F32Add,
    wasmparser::Operator::F32Sub => wasm_encoder::Instruction::F32Sub,
    wasmparser::Operator::F32Mul => wasm_encoder::Instruction::F32Mul,
    wasmparser::Operator::F32Div => wasm_encoder::Instruction::F32Div,
    wasmparser::Operator::F32Min => wasm_encoder::Instruction::F32Min,
    wasmparser::Operator::F32Max => wasm_encoder::Instruction::F32Max,
    wasmparser::Operator::F32Copysign => wasm_encoder::Instruction::F32Copysign,
    wasmparser::Operator::F64Abs => wasm_encoder::Instruction::F64Abs,
    wasmparser::Operator::F64Neg => wasm_encoder::Instruction::F64Neg,
    wasmparser::Operator::F64Ceil => wasm_encoder::Instruction::F64Ceil,
    wasmparser::Operator::F64Floor => wasm_encoder::Instruction::F64Floor,
    wasmparser::Operator::F64Trunc => wasm_encoder::Instruction::F64Trunc,
    wasmparser::Operator::F64Nearest => wasm_encoder::Instruction::F64Nearest,
    wasmparser::Operator::F64Sqrt => wasm_encoder::Instruction::F64Sqrt,
    wasmparser::Operator::F64Add => wasm_encoder::Instruction::F64Add,
    wasmparser::Operator::F64Sub => wasm_encoder::Instruction::F64Sub,
    wasmparser::Operator::F64Mul => wasm_encoder::Instruction::F64Mul,
    wasmparser::Operator::F64Div => wasm_encoder::Instruction::F64Div,
    wasmparser::Operator::F64Min => wasm_encoder::Instruction::F64Min,
    wasmparser::Operator::F64Max => wasm_encoder::Instruction::F64Max,
    wasmparser::Operator::F64Copysign => wasm_encoder::Instruction::F64Copysign,
    wasmparser::Operator::I32WrapI64 => wasm_encoder::Instruction::I32WrapI64,
    wasmparser::Operator::I32TruncF32S => wasm_encoder::Instruction::I32TruncF32S,
    wasmparser::Operator::I32TruncF32U => wasm_encoder::Instruction::I32TruncF32U,
    wasmparser::Operator::I32TruncF64S => wasm_encoder::Instruction::I32TruncF64S,
    wasmparser::Operator::I32TruncF64U => wasm_encoder::Instruction::I32TruncF64U,
    wasmparser::Operator::I64ExtendI32S => wasm_encoder::Instruction::I64ExtendI32S,
    wasmparser::Operator::I64ExtendI32U => wasm_encoder::Instruction::I64ExtendI32U,
    wasmparser::Operator::I64TruncF32S => wasm_encoder::Instruction::I64TruncF32S,
    wasmparser::Operator::I64TruncF32U => wasm_encoder::Instruction::I64TruncF32U,
    wasmparser::Operator::I64TruncF64S => wasm_encoder::Instruction::I64TruncF64S,
    wasmparser::Operator::I64TruncF64U => wasm_encoder::Instruction::I64TruncF64U,
    wasmparser::Operator::F32ConvertI32S => wasm_encoder::Instruction::F32ConvertI32S,
    wasmparser::Operator::F32ConvertI32U => wasm_encoder::Instruction::F32ConvertI32U,
    wasmparser::Operator::F32ConvertI64S => wasm_encoder::Instruction::F32ConvertI64S,
    wasmparser::Operator::F32ConvertI64U => wasm_encoder::Instruction::F32ConvertI64U,
    wasmparser::Operator::F32DemoteF64 => wasm_encoder::Instruction::F32DemoteF64,
    wasmparser::Operator::F64ConvertI32S => wasm_encoder::Instruction::F64ConvertI32S,
    wasmparser::Operator::F64ConvertI32U => wasm_encoder::Instruction::F64ConvertI32U,
    wasmparser::Operator::F64ConvertI64S => wasm_encoder::Instruction::F64ConvertI64S,
    wasmparser::Operator::F64ConvertI64U => wasm_encoder::Instruction::F64ConvertI64U,
    wasmparser::Operator::F64PromoteF32 => wasm_encoder::Instruction::F64PromoteF32,
    wasmparser::Operator::I32ReinterpretF32 => wasm_encoder::Instruction::I32ReinterpretF32,
    wasmparser::Operator::I64ReinterpretF64 => wasm_encoder::Instruction::I64ReinterpretF64,
    wasmparser::Operator::F32ReinterpretI32 => wasm_encoder::Instruction::F32ReinterpretI32,
    wasmparser::Operator::F64ReinterpretI64 => wasm_encoder::Instruction::F64ReinterpretI64,
    wasmparser::Operator::I32Extend8S => wasm_encoder::Instruction::I32Extend8S,
    wasmparser::Operator::I32Extend16S => wasm_encoder::Instruction::I32Extend16S,
    wasmparser::Operator::I64Extend8S => wasm_encoder::Instruction::I64Extend8S,
    wasmparser::Operator::I64Extend16S => wasm_encoder::Instruction::I64Extend16S,
    wasmparser::Operator::I64Extend32S => wasm_encoder::Instruction::I64Extend32S,
    wasmparser::Operator::RefEq => wasm_encoder::Instruction::RefEq,
    wasmparser::Operator::StructNew { struct_type_index } => wasm_encoder::Instruction::StructNew(struct_type_index),
    wasmparser::Operator::StructNewDefault { struct_type_index } => wasm_encoder::Instruction::StructNewDefault(struct_type_index),
    wasmparser::Operator::StructGet { struct_type_index, field_index } => wasm_encoder::Instruction::StructGet { struct_type_index, field_index },
    wasmparser::Operator::StructGetS { struct_type_index, field_index } => wasm_encoder::Instruction::StructGetS { struct_type_index, field_index },
    wasmparser::Operator::StructGetU { struct_type_index, field_index } => wasm_encoder::Instruction::StructGetU { struct_type_index, field_index },
    wasmparser::Operator::StructSet { struct_type_index, field_index } => wasm_encoder::Instruction::StructSet { struct_type_index, field_index },
    wasmparser::Operator::ArrayNew { array_type_index } => wasm_encoder::Instruction::ArrayNew(array_type_index),
    wasmparser::Operator::ArrayNewDefault { array_type_index } => wasm_encoder::Instruction::ArrayNewDefault(array_type_index),
    wasmparser::Operator::ArrayNewFixed { array_type_index, array_size } => wasm_encoder::Instruction::ArrayNewFixed { array_type_index, array_size },
    wasmparser::Operator::ArrayNewData { array_type_index, array_data_index } => wasm_encoder::Instruction::ArrayNewData { array_type_index, array_data_index },
    wasmparser::Operator::ArrayNewElem { array_type_index, array_elem_index } => wasm_encoder::Instruction::ArrayNewElem { array_type_index, array_elem_index },
    wasmparser::Operator::ArrayGet { array_type_index } => wasm_encoder::Instruction::ArrayGet(array_type_index),
    wasmparser::Operator::ArrayGetS { array_type_index } => wasm_encoder::Instruction::ArrayGetS(array_type_index),
    wasmparser::Operator::ArrayGetU { array_type_index } => wasm_encoder::Instruction::ArrayGetU(array_type_index),
    wasmparser::Operator::ArraySet { array_type_index } => wasm_encoder::Instruction::ArraySet(array_type_index),
    wasmparser::Operator::ArrayLen => wasm_encoder::Instruction::ArrayLen,
    wasmparser::Operator::ArrayFill { array_type_index } => wasm_encoder::Instruction::ArrayFill(array_type_index),
    wasmparser::Operator::ArrayCopy { array_type_index_dst, array_type_index_src } => wasm_encoder::Instruction::ArrayCopy { array_type_index_dst, array_type_index_src },
    wasmparser::Operator::ArrayInitData { array_type_index, array_data_index } => wasm_encoder::Instruction::ArrayInitData { array_type_index, array_data_index },
    wasmparser::Operator::ArrayInitElem { array_type_index, array_elem_index } => wasm_encoder::Instruction::ArrayInitElem { array_type_index, array_elem_index },
    wasmparser::Operator::RefTestNonNull { hty } => wasm_encoder::Instruction::RefTestNonNull(map_heap_type(hty)),
    wasmparser::Operator::RefTestNullable { hty } => wasm_encoder::Instruction::RefTestNullable(map_heap_type(hty)),
    wasmparser::Operator::RefCastNonNull { hty } => wasm_encoder::Instruction::RefCastNonNull(map_heap_type(hty)),
    wasmparser::Operator::RefCastNullable { hty } => wasm_encoder::Instruction::RefCastNullable(map_heap_type(hty)),
    wasmparser::Operator::BrOnCast { relative_depth, from_ref_type, to_ref_type } => wasm_encoder::Instruction::BrOnCast { relative_depth, from_ref_type: map_ref_type(from_ref_type), to_ref_type: map_ref_type(to_ref_type) },
    wasmparser::Operator::BrOnCastFail { relative_depth, from_ref_type, to_ref_type } => wasm_encoder::Instruction::BrOnCastFail { relative_depth, from_ref_type: map_ref_type(from_ref_type), to_ref_type: map_ref_type(to_ref_type) },
    wasmparser::Operator::AnyConvertExtern => wasm_encoder::Instruction::AnyConvertExtern,
    wasmparser::Operator::ExternConvertAny => wasm_encoder::Instruction::ExternConvertAny,
    wasmparser::Operator::RefI31 => wasm_encoder::Instruction::RefI31,
    wasmparser::Operator::I31GetS => wasm_encoder::Instruction::I31GetS,
    wasmparser::Operator::I31GetU => wasm_encoder::Instruction::I31GetU,
    wasmparser::Operator::I32TruncSatF32S => wasm_encoder::Instruction::I32TruncSatF32S,
    wasmparser::Operator::I32TruncSatF32U => wasm_encoder::Instruction::I32TruncSatF32U,
    wasmparser::Operator::I32TruncSatF64S => wasm_encoder::Instruction::I32TruncSatF64S,
    wasmparser::Operator::I32TruncSatF64U => wasm_encoder::Instruction::I32TruncSatF64U,
    wasmparser::Operator::I64TruncSatF32S => wasm_encoder::Instruction::I64TruncSatF32S,
    wasmparser::Operator::I64TruncSatF32U => wasm_encoder::Instruction::I64TruncSatF32U,
    wasmparser::Operator::I64TruncSatF64S => wasm_encoder::Instruction::I64TruncSatF64S,
    wasmparser::Operator::I64TruncSatF64U => wasm_encoder::Instruction::I64TruncSatF64U,
    wasmparser::Operator::MemoryInit { data_index, mem } => wasm_encoder::Instruction::MemoryInit { mem, data_index },
    wasmparser::Operator::DataDrop { data_index } => wasm_encoder::Instruction::DataDrop(data_index),
    wasmparser::Operator::MemoryCopy { dst_mem, src_mem } => wasm_encoder::Instruction::MemoryCopy { dst_mem, src_mem },
    wasmparser::Operator::MemoryFill { mem } => wasm_encoder::Instruction::MemoryFill(mem),
    wasmparser::Operator::TableInit { elem_index, table } => wasm_encoder::Instruction::TableInit { elem_index, table },
    wasmparser::Operator::ElemDrop { elem_index } => wasm_encoder::Instruction::ElemDrop(elem_index),
    wasmparser::Operator::TableCopy { dst_table, src_table } => wasm_encoder::Instruction::TableCopy { dst_table, src_table },
    wasmparser::Operator::TypedSelect { ty } => wasm_encoder::Instruction::TypedSelect(map_val_type(ty)),
    wasmparser::Operator::TypedSelectMulti { tys } => wasm_encoder::Instruction::TypedSelectMulti(Cow::Owned(map_val_types(tys))),
    wasmparser::Operator::RefNull { hty } => wasm_encoder::Instruction::RefNull(map_heap_type(hty)),
    wasmparser::Operator::RefIsNull => wasm_encoder::Instruction::RefIsNull,
    wasmparser::Operator::RefFunc { function_index } => wasm_encoder::Instruction::RefFunc(function_index),
    wasmparser::Operator::TableFill { table } => wasm_encoder::Instruction::TableFill(table),
    wasmparser::Operator::TableGet { table } => wasm_encoder::Instruction::TableGet(table),
    wasmparser::Operator::TableSet { table } => wasm_encoder::Instruction::TableSet(table),
    wasmparser::Operator::TableGrow { table } => wasm_encoder::Instruction::TableGrow(table),
    wasmparser::Operator::TableSize { table } => wasm_encoder::Instruction::TableSize(table),
    wasmparser::Operator::ReturnCall { function_index } => wasm_encoder::Instruction::ReturnCall(function_index),
    wasmparser::Operator::ReturnCallIndirect { type_index, table_index } => wasm_encoder::Instruction::ReturnCallIndirect { type_index, table_index },
    wasmparser::Operator::MemoryDiscard { mem } => wasm_encoder::Instruction::MemoryDiscard(mem),
    wasmparser::Operator::MemoryAtomicNotify { memarg } => wasm_encoder::Instruction::MemoryAtomicNotify(map_mem_arg(memarg)),
    wasmparser::Operator::MemoryAtomicWait32 { memarg } => wasm_encoder::Instruction::MemoryAtomicWait32(map_mem_arg(memarg)),
    wasmparser::Operator::MemoryAtomicWait64 { memarg } => wasm_encoder::Instruction::MemoryAtomicWait64(map_mem_arg(memarg)),
    wasmparser::Operator::AtomicFence => wasm_encoder::Instruction::AtomicFence,
    wasmparser::Operator::I32AtomicLoad { memarg } => wasm_encoder::Instruction::I32AtomicLoad(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicLoad { memarg } => wasm_encoder::Instruction::I64AtomicLoad(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicLoad8U { memarg } => wasm_encoder::Instruction::I32AtomicLoad8U(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicLoad16U { memarg } => wasm_encoder::Instruction::I32AtomicLoad16U(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicLoad8U { memarg } => wasm_encoder::Instruction::I64AtomicLoad8U(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicLoad16U { memarg } => wasm_encoder::Instruction::I64AtomicLoad16U(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicLoad32U { memarg } => wasm_encoder::Instruction::I64AtomicLoad32U(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicStore { memarg } => wasm_encoder::Instruction::I32AtomicStore(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicStore { memarg } => wasm_encoder::Instruction::I64AtomicStore(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicStore8 { memarg } => wasm_encoder::Instruction::I32AtomicStore8(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicStore16 { memarg } => wasm_encoder::Instruction::I32AtomicStore16(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicStore8 { memarg } => wasm_encoder::Instruction::I64AtomicStore8(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicStore16 { memarg } => wasm_encoder::Instruction::I64AtomicStore16(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicStore32 { memarg } => wasm_encoder::Instruction::I64AtomicStore32(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwAdd { memarg } => wasm_encoder::Instruction::I32AtomicRmwAdd(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwAdd { memarg } => wasm_encoder::Instruction::I64AtomicRmwAdd(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8AddU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8AddU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16AddU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16AddU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8AddU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8AddU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16AddU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16AddU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32AddU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32AddU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwSub { memarg } => wasm_encoder::Instruction::I32AtomicRmwSub(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwSub { memarg } => wasm_encoder::Instruction::I64AtomicRmwSub(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8SubU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8SubU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16SubU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16SubU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8SubU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8SubU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16SubU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16SubU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32SubU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32SubU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwAnd { memarg } => wasm_encoder::Instruction::I32AtomicRmwAnd(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwAnd { memarg } => wasm_encoder::Instruction::I64AtomicRmwAnd(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8AndU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8AndU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16AndU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16AndU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8AndU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8AndU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16AndU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16AndU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32AndU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32AndU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwOr { memarg } => wasm_encoder::Instruction::I32AtomicRmwOr(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwOr { memarg } => wasm_encoder::Instruction::I64AtomicRmwOr(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8OrU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8OrU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16OrU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16OrU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8OrU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8OrU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16OrU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16OrU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32OrU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32OrU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwXor { memarg } => wasm_encoder::Instruction::I32AtomicRmwXor(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwXor { memarg } => wasm_encoder::Instruction::I64AtomicRmwXor(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8XorU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8XorU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16XorU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16XorU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8XorU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8XorU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16XorU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16XorU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32XorU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32XorU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwXchg { memarg } => wasm_encoder::Instruction::I32AtomicRmwXchg(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwXchg { memarg } => wasm_encoder::Instruction::I64AtomicRmwXchg(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8XchgU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8XchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16XchgU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16XchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8XchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8XchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16XchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16XchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32XchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32XchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmwCmpxchg { memarg } => wasm_encoder::Instruction::I32AtomicRmwCmpxchg(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmwCmpxchg { memarg } => wasm_encoder::Instruction::I64AtomicRmwCmpxchg(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw8CmpxchgU { memarg } => wasm_encoder::Instruction::I32AtomicRmw8CmpxchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I32AtomicRmw16CmpxchgU { memarg } => wasm_encoder::Instruction::I32AtomicRmw16CmpxchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw8CmpxchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw8CmpxchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw16CmpxchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw16CmpxchgU(map_mem_arg(memarg)),
    wasmparser::Operator::I64AtomicRmw32CmpxchgU { memarg } => wasm_encoder::Instruction::I64AtomicRmw32CmpxchgU(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load { memarg } => wasm_encoder::Instruction::V128Load(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load8x8S { memarg } => wasm_encoder::Instruction::V128Load8x8S(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load8x8U { memarg } => wasm_encoder::Instruction::V128Load8x8U(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load16x4S { memarg } => wasm_encoder::Instruction::V128Load16x4S(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load16x4U { memarg } => wasm_encoder::Instruction::V128Load16x4U(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load32x2S { memarg } => wasm_encoder::Instruction::V128Load32x2S(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load32x2U { memarg } => wasm_encoder::Instruction::V128Load32x2U(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load8Splat { memarg } => wasm_encoder::Instruction::V128Load8Splat(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load16Splat { memarg } => wasm_encoder::Instruction::V128Load16Splat(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load32Splat { memarg } => wasm_encoder::Instruction::V128Load32Splat(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load64Splat { memarg } => wasm_encoder::Instruction::V128Load64Splat(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load32Zero { memarg } => wasm_encoder::Instruction::V128Load32Zero(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load64Zero { memarg } => wasm_encoder::Instruction::V128Load64Zero(map_mem_arg(memarg)),
    wasmparser::Operator::V128Store { memarg } => wasm_encoder::Instruction::V128Store(map_mem_arg(memarg)),
    wasmparser::Operator::V128Load8Lane { memarg, lane } => wasm_encoder::Instruction::V128Load8Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Load16Lane { memarg, lane } => wasm_encoder::Instruction::V128Load16Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Load32Lane { memarg, lane } => wasm_encoder::Instruction::V128Load32Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Load64Lane { memarg, lane } => wasm_encoder::Instruction::V128Load64Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Store8Lane { memarg, lane } => wasm_encoder::Instruction::V128Store8Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Store16Lane { memarg, lane } => wasm_encoder::Instruction::V128Store16Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Store32Lane { memarg, lane } => wasm_encoder::Instruction::V128Store32Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Store64Lane { memarg, lane } => wasm_encoder::Instruction::V128Store64Lane { memarg: map_mem_arg(memarg), lane },
    wasmparser::Operator::V128Const { value } => wasm_encoder::Instruction::V128Const(value.i128()),
    wasmparser::Operator::I8x16Shuffle { lanes } => wasm_encoder::Instruction::I8x16Shuffle(lanes),
    wasmparser::Operator::I8x16ExtractLaneS { lane } => wasm_encoder::Instruction::I8x16ExtractLaneS(lane),
    wasmparser::Operator::I8x16ExtractLaneU { lane } => wasm_encoder::Instruction::I8x16ExtractLaneU(lane),
    wasmparser::Operator::I8x16ReplaceLane { lane } => wasm_encoder::Instruction::I8x16ReplaceLane(lane),
    wasmparser::Operator::I16x8ExtractLaneS { lane } => wasm_encoder::Instruction::I16x8ExtractLaneS(lane),
    wasmparser::Operator::I16x8ExtractLaneU { lane } => wasm_encoder::Instruction::I16x8ExtractLaneU(lane),
    wasmparser::Operator::I16x8ReplaceLane { lane } => wasm_encoder::Instruction::I16x8ReplaceLane(lane),
    wasmparser::Operator::I32x4ExtractLane { lane } => wasm_encoder::Instruction::I32x4ExtractLane(lane),
    wasmparser::Operator::I32x4ReplaceLane { lane } => wasm_encoder::Instruction::I32x4ReplaceLane(lane),
    wasmparser::Operator::I64x2ExtractLane { lane } => wasm_encoder::Instruction::I64x2ExtractLane(lane),
    wasmparser::Operator::I64x2ReplaceLane { lane } => wasm_encoder::Instruction::I64x2ReplaceLane(lane),
    wasmparser::Operator::F32x4ExtractLane { lane } => wasm_encoder::Instruction::F32x4ExtractLane(lane),
    wasmparser::Operator::F32x4ReplaceLane { lane } => wasm_encoder::Instruction::F32x4ReplaceLane(lane),
    wasmparser::Operator::F64x2ExtractLane { lane } => wasm_encoder::Instruction::F64x2ExtractLane(lane),
    wasmparser::Operator::F64x2ReplaceLane { lane } => wasm_encoder::Instruction::F64x2ReplaceLane(lane),
    wasmparser::Operator::I8x16Swizzle => wasm_encoder::Instruction::I8x16Swizzle,
    wasmparser::Operator::I8x16Splat => wasm_encoder::Instruction::I8x16Splat,
    wasmparser::Operator::I16x8Splat => wasm_encoder::Instruction::I16x8Splat,
    wasmparser::Operator::I32x4Splat => wasm_encoder::Instruction::I32x4Splat,
    wasmparser::Operator::I64x2Splat => wasm_encoder::Instruction::I64x2Splat,
    wasmparser::Operator::F32x4Splat => wasm_encoder::Instruction::F32x4Splat,
    wasmparser::Operator::F64x2Splat => wasm_encoder::Instruction::F64x2Splat,
    wasmparser::Operator::I8x16Eq => wasm_encoder::Instruction::I8x16Eq,
    wasmparser::Operator::I8x16Ne => wasm_encoder::Instruction::I8x16Ne,
    wasmparser::Operator::I8x16LtS => wasm_encoder::Instruction::I8x16LtS,
    wasmparser::Operator::I8x16LtU => wasm_encoder::Instruction::I8x16LtU,
    wasmparser::Operator::I8x16GtS => wasm_encoder::Instruction::I8x16GtS,
    wasmparser::Operator::I8x16GtU => wasm_encoder::Instruction::I8x16GtU,
    wasmparser::Operator::I8x16LeS => wasm_encoder::Instruction::I8x16LeS,
    wasmparser::Operator::I8x16LeU => wasm_encoder::Instruction::I8x16LeU,
    wasmparser::Operator::I8x16GeS => wasm_encoder::Instruction::I8x16GeS,
    wasmparser::Operator::I8x16GeU => wasm_encoder::Instruction::I8x16GeU,
    wasmparser::Operator::I16x8Eq => wasm_encoder::Instruction::I16x8Eq,
    wasmparser::Operator::I16x8Ne => wasm_encoder::Instruction::I16x8Ne,
    wasmparser::Operator::I16x8LtS => wasm_encoder::Instruction::I16x8LtS,
    wasmparser::Operator::I16x8LtU => wasm_encoder::Instruction::I16x8LtU,
    wasmparser::Operator::I16x8GtS => wasm_encoder::Instruction::I16x8GtS,
    wasmparser::Operator::I16x8GtU => wasm_encoder::Instruction::I16x8GtU,
    wasmparser::Operator::I16x8LeS => wasm_encoder::Instruction::I16x8LeS,
    wasmparser::Operator::I16x8LeU => wasm_encoder::Instruction::I16x8LeU,
    wasmparser::Operator::I16x8GeS => wasm_encoder::Instruction::I16x8GeS,
    wasmparser::Operator::I16x8GeU => wasm_encoder::Instruction::I16x8GeU,
    wasmparser::Operator::I32x4Eq => wasm_encoder::Instruction::I32x4Eq,
    wasmparser::Operator::I32x4Ne => wasm_encoder::Instruction::I32x4Ne,
    wasmparser::Operator::I32x4LtS => wasm_encoder::Instruction::I32x4LtS,
    wasmparser::Operator::I32x4LtU => wasm_encoder::Instruction::I32x4LtU,
    wasmparser::Operator::I32x4GtS => wasm_encoder::Instruction::I32x4GtS,
    wasmparser::Operator::I32x4GtU => wasm_encoder::Instruction::I32x4GtU,
    wasmparser::Operator::I32x4LeS => wasm_encoder::Instruction::I32x4LeS,
    wasmparser::Operator::I32x4LeU => wasm_encoder::Instruction::I32x4LeU,
    wasmparser::Operator::I32x4GeS => wasm_encoder::Instruction::I32x4GeS,
    wasmparser::Operator::I32x4GeU => wasm_encoder::Instruction::I32x4GeU,
    wasmparser::Operator::I64x2Eq => wasm_encoder::Instruction::I64x2Eq,
    wasmparser::Operator::I64x2Ne => wasm_encoder::Instruction::I64x2Ne,
    wasmparser::Operator::I64x2LtS => wasm_encoder::Instruction::I64x2LtS,
    wasmparser::Operator::I64x2GtS => wasm_encoder::Instruction::I64x2GtS,
    wasmparser::Operator::I64x2LeS => wasm_encoder::Instruction::I64x2LeS,
    wasmparser::Operator::I64x2GeS => wasm_encoder::Instruction::I64x2GeS,
    wasmparser::Operator::F32x4Eq => wasm_encoder::Instruction::F32x4Eq,
    wasmparser::Operator::F32x4Ne => wasm_encoder::Instruction::F32x4Ne,
    wasmparser::Operator::F32x4Lt => wasm_encoder::Instruction::F32x4Lt,
    wasmparser::Operator::F32x4Gt => wasm_encoder::Instruction::F32x4Gt,
    wasmparser::Operator::F32x4Le => wasm_encoder::Instruction::F32x4Le,
    wasmparser::Operator::F32x4Ge => wasm_encoder::Instruction::F32x4Ge,
    wasmparser::Operator::F64x2Eq => wasm_encoder::Instruction::F64x2Eq,
    wasmparser::Operator::F64x2Ne => wasm_encoder::Instruction::F64x2Ne,
    wasmparser::Operator::F64x2Lt => wasm_encoder::Instruction::F64x2Lt,
    wasmparser::Operator::F64x2Gt => wasm_encoder::Instruction::F64x2Gt,
    wasmparser::Operator::F64x2Le => wasm_encoder::Instruction::F64x2Le,
    wasmparser::Operator::F64x2Ge => wasm_encoder::Instruction::F64x2Ge,
    wasmparser::Operator::V128Not => wasm_encoder::Instruction::V128Not,
    wasmparser::Operator::V128And => wasm_encoder::Instruction::V128And,
    wasmparser::Operator::V128AndNot => wasm_encoder::Instruction::V128AndNot,
    wasmparser::Operator::V128Or => wasm_encoder::Instruction::V128Or,
    wasmparser::Operator::V128Xor => wasm_encoder::Instruction::V128Xor,
    wasmparser::Operator::V128Bitselect => wasm_encoder::Instruction::V128Bitselect,
    wasmparser::Operator::V128AnyTrue => wasm_encoder::Instruction::V128AnyTrue,
    wasmparser::Operator::I8x16Abs => wasm_encoder::Instruction::I8x16Abs,
    wasmparser::Operator::I8x16Neg => wasm_encoder::Instruction::I8x16Neg,
    wasmparser::Operator::I8x16Popcnt => wasm_encoder::Instruction::I8x16Popcnt,
    wasmparser::Operator::I8x16AllTrue => wasm_encoder::Instruction::I8x16AllTrue,
    wasmparser::Operator::I8x16Bitmask => wasm_encoder::Instruction::I8x16Bitmask,
    wasmparser::Operator::I8x16NarrowI16x8S => wasm_encoder::Instruction::I8x16NarrowI16x8S,
    wasmparser::Operator::I8x16NarrowI16x8U => wasm_encoder::Instruction::I8x16NarrowI16x8U,
    wasmparser::Operator::I8x16Shl => wasm_encoder::Instruction::I8x16Shl,
    wasmparser::Operator::I8x16ShrS => wasm_encoder::Instruction::I8x16ShrS,
    wasmparser::Operator::I8x16ShrU => wasm_encoder::Instruction::I8x16ShrU,
    wasmparser::Operator::I8x16Add => wasm_encoder::Instruction::I8x16Add,
    wasmparser::Operator::I8x16AddSatS => wasm_encoder::Instruction::I8x16AddSatS,
    wasmparser::Operator::I8x16AddSatU => wasm_encoder::Instruction::I8x16AddSatU,
    wasmparser::Operator::I8x16Sub => wasm_encoder::Instruction::I8x16Sub,
    wasmparser::Operator::I8x16SubSatS => wasm_encoder::Instruction::I8x16SubSatS,
    wasmparser::Operator::I8x16SubSatU => wasm_encoder::Instruction::I8x16SubSatU,
    wasmparser::Operator::I8x16MinS => wasm_encoder::Instruction::I8x16MinS,
    wasmparser::Operator::I8x16MinU => wasm_encoder::Instruction::I8x16MinU,
    wasmparser::Operator::I8x16MaxS => wasm_encoder::Instruction::I8x16MaxS,
    wasmparser::Operator::I8x16MaxU => wasm_encoder::Instruction::I8x16MaxU,
    wasmparser::Operator::I8x16AvgrU => wasm_encoder::Instruction::I8x16AvgrU,
    wasmparser::Operator::I16x8ExtAddPairwiseI8x16S => wasm_encoder::Instruction::I16x8ExtAddPairwiseI8x16S,
    wasmparser::Operator::I16x8ExtAddPairwiseI8x16U => wasm_encoder::Instruction::I16x8ExtAddPairwiseI8x16U,
    wasmparser::Operator::I16x8Abs => wasm_encoder::Instruction::I16x8Abs,
    wasmparser::Operator::I16x8Neg => wasm_encoder::Instruction::I16x8Neg,
    wasmparser::Operator::I16x8Q15MulrSatS => wasm_encoder::Instruction::I16x8Q15MulrSatS,
    wasmparser::Operator::I16x8AllTrue => wasm_encoder::Instruction::I16x8AllTrue,
    wasmparser::Operator::I16x8Bitmask => wasm_encoder::Instruction::I16x8Bitmask,
    wasmparser::Operator::I16x8NarrowI32x4S => wasm_encoder::Instruction::I16x8NarrowI32x4S,
    wasmparser::Operator::I16x8NarrowI32x4U => wasm_encoder::Instruction::I16x8NarrowI32x4U,
    wasmparser::Operator::I16x8ExtendLowI8x16S => wasm_encoder::Instruction::I16x8ExtendLowI8x16S,
    wasmparser::Operator::I16x8ExtendHighI8x16S => wasm_encoder::Instruction::I16x8ExtendHighI8x16S,
    wasmparser::Operator::I16x8ExtendLowI8x16U => wasm_encoder::Instruction::I16x8ExtendLowI8x16U,
    wasmparser::Operator::I16x8ExtendHighI8x16U => wasm_encoder::Instruction::I16x8ExtendHighI8x16U,
    wasmparser::Operator::I16x8Shl => wasm_encoder::Instruction::I16x8Shl,
    wasmparser::Operator::I16x8ShrS => wasm_encoder::Instruction::I16x8ShrS,
    wasmparser::Operator::I16x8ShrU => wasm_encoder::Instruction::I16x8ShrU,
    wasmparser::Operator::I16x8Add => wasm_encoder::Instruction::I16x8Add,
    wasmparser::Operator::I16x8AddSatS => wasm_encoder::Instruction::I16x8AddSatS,
    wasmparser::Operator::I16x8AddSatU => wasm_encoder::Instruction::I16x8AddSatU,
    wasmparser::Operator::I16x8Sub => wasm_encoder::Instruction::I16x8Sub,
    wasmparser::Operator::I16x8SubSatS => wasm_encoder::Instruction::I16x8SubSatS,
    wasmparser::Operator::I16x8SubSatU => wasm_encoder::Instruction::I16x8SubSatU,
    wasmparser::Operator::I16x8Mul => wasm_encoder::Instruction::I16x8Mul,
    wasmparser::Operator::I16x8MinS => wasm_encoder::Instruction::I16x8MinS,
    wasmparser::Operator::I16x8MinU => wasm_encoder::Instruction::I16x8MinU,
    wasmparser::Operator::I16x8MaxS => wasm_encoder::Instruction::I16x8MaxS,
    wasmparser::Operator::I16x8MaxU => wasm_encoder::Instruction::I16x8MaxU,
    wasmparser::Operator::I16x8AvgrU => wasm_encoder::Instruction::I16x8AvgrU,
    wasmparser::Operator::I16x8ExtMulLowI8x16S => wasm_encoder::Instruction::I16x8ExtMulLowI8x16S,
    wasmparser::Operator::I16x8ExtMulHighI8x16S => wasm_encoder::Instruction::I16x8ExtMulHighI8x16S,
    wasmparser::Operator::I16x8ExtMulLowI8x16U => wasm_encoder::Instruction::I16x8ExtMulLowI8x16U,
    wasmparser::Operator::I16x8ExtMulHighI8x16U => wasm_encoder::Instruction::I16x8ExtMulHighI8x16U,
    wasmparser::Operator::I32x4ExtAddPairwiseI16x8S => wasm_encoder::Instruction::I32x4ExtAddPairwiseI16x8S,
    wasmparser::Operator::I32x4ExtAddPairwiseI16x8U => wasm_encoder::Instruction::I32x4ExtAddPairwiseI16x8U,
    wasmparser::Operator::I32x4Abs => wasm_encoder::Instruction::I32x4Abs,
    wasmparser::Operator::I32x4Neg => wasm_encoder::Instruction::I32x4Neg,
    wasmparser::Operator::I32x4AllTrue => wasm_encoder::Instruction::I32x4AllTrue,
    wasmparser::Operator::I32x4Bitmask => wasm_encoder::Instruction::I32x4Bitmask,
    wasmparser::Operator::I32x4ExtendLowI16x8S => wasm_encoder::Instruction::I32x4ExtendLowI16x8S,
    wasmparser::Operator::I32x4ExtendHighI16x8S => wasm_encoder::Instruction::I32x4ExtendHighI16x8S,
    wasmparser::Operator::I32x4ExtendLowI16x8U => wasm_encoder::Instruction::I32x4ExtendLowI16x8U,
    wasmparser::Operator::I32x4ExtendHighI16x8U => wasm_encoder::Instruction::I32x4ExtendHighI16x8U,
    wasmparser::Operator::I32x4Shl => wasm_encoder::Instruction::I32x4Shl,
    wasmparser::Operator::I32x4ShrS => wasm_encoder::Instruction::I32x4ShrS,
    wasmparser::Operator::I32x4ShrU => wasm_encoder::Instruction::I32x4ShrU,
    wasmparser::Operator::I32x4Add => wasm_encoder::Instruction::I32x4Add,
    wasmparser::Operator::I32x4Sub => wasm_encoder::Instruction::I32x4Sub,
    wasmparser::Operator::I32x4Mul => wasm_encoder::Instruction::I32x4Mul,
    wasmparser::Operator::I32x4MinS => wasm_encoder::Instruction::I32x4MinS,
    wasmparser::Operator::I32x4MinU => wasm_encoder::Instruction::I32x4MinU,
    wasmparser::Operator::I32x4MaxS => wasm_encoder::Instruction::I32x4MaxS,
    wasmparser::Operator::I32x4MaxU => wasm_encoder::Instruction::I32x4MaxU,
    wasmparser::Operator::I32x4DotI16x8S => wasm_encoder::Instruction::I32x4DotI16x8S,
    wasmparser::Operator::I32x4ExtMulLowI16x8S => wasm_encoder::Instruction::I32x4ExtMulLowI16x8S,
    wasmparser::Operator::I32x4ExtMulHighI16x8S => wasm_encoder::Instruction::I32x4ExtMulHighI16x8S,
    wasmparser::Operator::I32x4ExtMulLowI16x8U => wasm_encoder::Instruction::I32x4ExtMulLowI16x8U,
    wasmparser::Operator::I32x4ExtMulHighI16x8U => wasm_encoder::Instruction::I32x4ExtMulHighI16x8U,
    wasmparser::Operator::I64x2Abs => wasm_encoder::Instruction::I64x2Abs,
    wasmparser::Operator::I64x2Neg => wasm_encoder::Instruction::I64x2Neg,
    wasmparser::Operator::I64x2AllTrue => wasm_encoder::Instruction::I64x2AllTrue,
    wasmparser::Operator::I64x2Bitmask => wasm_encoder::Instruction::I64x2Bitmask,
    wasmparser::Operator::I64x2ExtendLowI32x4S => wasm_encoder::Instruction::I64x2ExtendLowI32x4S,
    wasmparser::Operator::I64x2ExtendHighI32x4S => wasm_encoder::Instruction::I64x2ExtendHighI32x4S,
    wasmparser::Operator::I64x2ExtendLowI32x4U => wasm_encoder::Instruction::I64x2ExtendLowI32x4U,
    wasmparser::Operator::I64x2ExtendHighI32x4U => wasm_encoder::Instruction::I64x2ExtendHighI32x4U,
    wasmparser::Operator::I64x2Shl => wasm_encoder::Instruction::I64x2Shl,
    wasmparser::Operator::I64x2ShrS => wasm_encoder::Instruction::I64x2ShrS,
    wasmparser::Operator::I64x2ShrU => wasm_encoder::Instruction::I64x2ShrU,
    wasmparser::Operator::I64x2Add => wasm_encoder::Instruction::I64x2Add,
    wasmparser::Operator::I64x2Sub => wasm_encoder::Instruction::I64x2Sub,
    wasmparser::Operator::I64x2Mul => wasm_encoder::Instruction::I64x2Mul,
    wasmparser::Operator::I64x2ExtMulLowI32x4S => wasm_encoder::Instruction::I64x2ExtMulLowI32x4S,
    wasmparser::Operator::I64x2ExtMulHighI32x4S => wasm_encoder::Instruction::I64x2ExtMulHighI32x4S,
    wasmparser::Operator::I64x2ExtMulLowI32x4U => wasm_encoder::Instruction::I64x2ExtMulLowI32x4U,
    wasmparser::Operator::I64x2ExtMulHighI32x4U => wasm_encoder::Instruction::I64x2ExtMulHighI32x4U,
    wasmparser::Operator::F32x4Ceil => wasm_encoder::Instruction::F32x4Ceil,
    wasmparser::Operator::F32x4Floor => wasm_encoder::Instruction::F32x4Floor,
    wasmparser::Operator::F32x4Trunc => wasm_encoder::Instruction::F32x4Trunc,
    wasmparser::Operator::F32x4Nearest => wasm_encoder::Instruction::F32x4Nearest,
    wasmparser::Operator::F32x4Abs => wasm_encoder::Instruction::F32x4Abs,
    wasmparser::Operator::F32x4Neg => wasm_encoder::Instruction::F32x4Neg,
    wasmparser::Operator::F32x4Sqrt => wasm_encoder::Instruction::F32x4Sqrt,
    wasmparser::Operator::F32x4Add => wasm_encoder::Instruction::F32x4Add,
    wasmparser::Operator::F32x4Sub => wasm_encoder::Instruction::F32x4Sub,
    wasmparser::Operator::F32x4Mul => wasm_encoder::Instruction::F32x4Mul,
    wasmparser::Operator::F32x4Div => wasm_encoder::Instruction::F32x4Div,
    wasmparser::Operator::F32x4Min => wasm_encoder::Instruction::F32x4Min,
    wasmparser::Operator::F32x4Max => wasm_encoder::Instruction::F32x4Max,
    wasmparser::Operator::F32x4PMin => wasm_encoder::Instruction::F32x4PMin,
    wasmparser::Operator::F32x4PMax => wasm_encoder::Instruction::F32x4PMax,
    wasmparser::Operator::F64x2Ceil => wasm_encoder::Instruction::F64x2Ceil,
    wasmparser::Operator::F64x2Floor => wasm_encoder::Instruction::F64x2Floor,
    wasmparser::Operator::F64x2Trunc => wasm_encoder::Instruction::F64x2Trunc,
    wasmparser::Operator::F64x2Nearest => wasm_encoder::Instruction::F64x2Nearest,
    wasmparser::Operator::F64x2Abs => wasm_encoder::Instruction::F64x2Abs,
    wasmparser::Operator::F64x2Neg => wasm_encoder::Instruction::F64x2Neg,
    wasmparser::Operator::F64x2Sqrt => wasm_encoder::Instruction::F64x2Sqrt,
    wasmparser::Operator::F64x2Add => wasm_encoder::Instruction::F64x2Add,
    wasmparser::Operator::F64x2Sub => wasm_encoder::Instruction::F64x2Sub,
    wasmparser::Operator::F64x2Mul => wasm_encoder::Instruction::F64x2Mul,
    wasmparser::Operator::F64x2Div => wasm_encoder::Instruction::F64x2Div,
    wasmparser::Operator::F64x2Min => wasm_encoder::Instruction::F64x2Min,
    wasmparser::Operator::F64x2Max => wasm_encoder::Instruction::F64x2Max,
    wasmparser::Operator::F64x2PMin => wasm_encoder::Instruction::F64x2PMin,
    wasmparser::Operator::F64x2PMax => wasm_encoder::Instruction::F64x2PMax,
    wasmparser::Operator::I32x4TruncSatF32x4S => wasm_encoder::Instruction::I32x4TruncSatF32x4S,
    wasmparser::Operator::I32x4TruncSatF32x4U => wasm_encoder::Instruction::I32x4TruncSatF32x4U,
    wasmparser::Operator::F32x4ConvertI32x4S => wasm_encoder::Instruction::F32x4ConvertI32x4S,
    wasmparser::Operator::F32x4ConvertI32x4U => wasm_encoder::Instruction::F32x4ConvertI32x4U,
    wasmparser::Operator::I32x4TruncSatF64x2SZero => wasm_encoder::Instruction::I32x4TruncSatF64x2SZero,
    wasmparser::Operator::I32x4TruncSatF64x2UZero => wasm_encoder::Instruction::I32x4TruncSatF64x2UZero,
    wasmparser::Operator::F64x2ConvertLowI32x4S => wasm_encoder::Instruction::F64x2ConvertLowI32x4S,
    wasmparser::Operator::F64x2ConvertLowI32x4U => wasm_encoder::Instruction::F64x2ConvertLowI32x4U,
    wasmparser::Operator::F32x4DemoteF64x2Zero => wasm_encoder::Instruction::F32x4DemoteF64x2Zero,
    wasmparser::Operator::F64x2PromoteLowF32x4 => wasm_encoder::Instruction::F64x2PromoteLowF32x4,
    wasmparser::Operator::I8x16RelaxedSwizzle => wasm_encoder::Instruction::I8x16RelaxedSwizzle,
    wasmparser::Operator::I32x4RelaxedTruncF32x4S => wasm_encoder::Instruction::I32x4RelaxedTruncF32x4S,
    wasmparser::Operator::I32x4RelaxedTruncF32x4U => wasm_encoder::Instruction::I32x4RelaxedTruncF32x4U,
    wasmparser::Operator::I32x4RelaxedTruncF64x2SZero => wasm_encoder::Instruction::I32x4RelaxedTruncF64x2SZero,
    wasmparser::Operator::I32x4RelaxedTruncF64x2UZero => wasm_encoder::Instruction::I32x4RelaxedTruncF64x2UZero,
    wasmparser::Operator::F32x4RelaxedMadd => wasm_encoder::Instruction::F32x4RelaxedMadd,
    wasmparser::Operator::F32x4RelaxedNmadd => wasm_encoder::Instruction::F32x4RelaxedNmadd,
    wasmparser::Operator::F64x2RelaxedMadd => wasm_encoder::Instruction::F64x2RelaxedMadd,
    wasmparser::Operator::F64x2RelaxedNmadd => wasm_encoder::Instruction::F64x2RelaxedNmadd,
    wasmparser::Operator::I8x16RelaxedLaneselect => wasm_encoder::Instruction::I8x16RelaxedLaneselect,
    wasmparser::Operator::I16x8RelaxedLaneselect => wasm_encoder::Instruction::I16x8RelaxedLaneselect,
    wasmparser::Operator::I32x4RelaxedLaneselect => wasm_encoder::Instruction::I32x4RelaxedLaneselect,
    wasmparser::Operator::I64x2RelaxedLaneselect => wasm_encoder::Instruction::I64x2RelaxedLaneselect,
    wasmparser::Operator::F32x4RelaxedMin => wasm_encoder::Instruction::F32x4RelaxedMin,
    wasmparser::Operator::F32x4RelaxedMax => wasm_encoder::Instruction::F32x4RelaxedMax,
    wasmparser::Operator::F64x2RelaxedMin => wasm_encoder::Instruction::F64x2RelaxedMin,
    wasmparser::Operator::F64x2RelaxedMax => wasm_encoder::Instruction::F64x2RelaxedMax,
    wasmparser::Operator::I16x8RelaxedQ15mulrS => wasm_encoder::Instruction::I16x8RelaxedQ15mulrS,
    wasmparser::Operator::I16x8RelaxedDotI8x16I7x16S => wasm_encoder::Instruction::I16x8RelaxedDotI8x16I7x16S,
    wasmparser::Operator::I32x4RelaxedDotI8x16I7x16AddS => wasm_encoder::Instruction::I32x4RelaxedDotI8x16I7x16AddS,
    wasmparser::Operator::TryTable { try_table } => wasm_encoder::Instruction::TryTable(map_block_type(try_table.ty), Cow::Owned(map_catches(try_table.catches))),
    wasmparser::Operator::Throw { tag_index } => wasm_encoder::Instruction::Throw(tag_index),
    wasmparser::Operator::ThrowRef => wasm_encoder::Instruction::ThrowRef,
    wasmparser::Operator::Try { blockty } => wasm_encoder::Instruction::Try(map_block_type(blockty)),
    wasmparser::Operator::Catch { tag_index } => wasm_encoder::Instruction::Catch(tag_index),
    wasmparser::Operator::Rethrow { relative_depth } => wasm_encoder::Instruction::Rethrow(relative_depth),
    wasmparser::Operator::Delegate { relative_depth } => wasm_encoder::Instruction::Delegate(relative_depth),
    wasmparser::Operator::CatchAll => wasm_encoder::Instruction::CatchAll,
    wasmparser::Operator::GlobalAtomicGet { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicGet { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicSet { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicSet { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwAdd { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwAdd { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwSub { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwSub { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwAnd { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwAnd { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwOr { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwOr { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwXor { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwXor { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwXchg { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwXchg { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::GlobalAtomicRmwCmpxchg { ordering, global_index } => wasm_encoder::Instruction::GlobalAtomicRmwCmpxchg { ordering: map_ordering(ordering), global_index },
    wasmparser::Operator::TableAtomicGet { ordering, table_index } => wasm_encoder::Instruction::TableAtomicGet { ordering: map_ordering(ordering), table_index },
    wasmparser::Operator::TableAtomicSet { ordering, table_index } => wasm_encoder::Instruction::TableAtomicSet { ordering: map_ordering(ordering), table_index },
    wasmparser::Operator::TableAtomicRmwXchg { ordering, table_index } => wasm_encoder::Instruction::TableAtomicRmwXchg { ordering: map_ordering(ordering), table_index },
    wasmparser::Operator::TableAtomicRmwCmpxchg { ordering, table_index } => wasm_encoder::Instruction::TableAtomicRmwCmpxchg { ordering: map_ordering(ordering), table_index },
    wasmparser::Operator::StructAtomicGet { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicGet { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicGetS { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicGetS { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicGetU { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicGetU { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicSet { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicSet { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwAdd { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwAdd { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwSub { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwSub { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwAnd { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwAnd { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwOr { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwOr { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwXor { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwXor { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwXchg { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwXchg { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::StructAtomicRmwCmpxchg { ordering, struct_type_index, field_index } => wasm_encoder::Instruction::StructAtomicRmwCmpxchg { ordering: map_ordering(ordering), struct_type_index, field_index },
    wasmparser::Operator::ArrayAtomicGet { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicGet { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicGetS { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicGetS { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicGetU { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicGetU { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicSet { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicSet { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwAdd { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwAdd { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwSub { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwSub { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwAnd { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwAnd { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwOr { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwOr { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwXor { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwXor { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwXchg { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwXchg { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::ArrayAtomicRmwCmpxchg { ordering, array_type_index } => wasm_encoder::Instruction::ArrayAtomicRmwCmpxchg { ordering: map_ordering(ordering), array_type_index },
    wasmparser::Operator::RefI31Shared => wasm_encoder::Instruction::RefI31Shared,
    wasmparser::Operator::CallRef { type_index } => wasm_encoder::Instruction::CallRef(type_index),
    wasmparser::Operator::ReturnCallRef { type_index } => wasm_encoder::Instruction::ReturnCallRef(type_index),
    wasmparser::Operator::RefAsNonNull => wasm_encoder::Instruction::RefAsNonNull,
    wasmparser::Operator::BrOnNull { relative_depth } => wasm_encoder::Instruction::BrOnNull(relative_depth),
    wasmparser::Operator::BrOnNonNull { relative_depth } => wasm_encoder::Instruction::BrOnNonNull(relative_depth),
    wasmparser::Operator::ContNew { cont_type_index } => wasm_encoder::Instruction::ContNew(cont_type_index),
    wasmparser::Operator::ContBind { argument_index, result_index } => wasm_encoder::Instruction::ContBind { argument_index, result_index },
    wasmparser::Operator::Suspend { tag_index } => wasm_encoder::Instruction::Suspend(tag_index),
    wasmparser::Operator::Resume { cont_type_index, resume_table } => wasm_encoder::Instruction::Resume { cont_type_index, resume_table: Cow::Owned(map_resume_table(resume_table)) },
    wasmparser::Operator::ResumeThrow { cont_type_index, tag_index, resume_table } => wasm_encoder::Instruction::ResumeThrow { cont_type_index, tag_index, resume_table: Cow::Owned(map_resume_table(resume_table)) },
    wasmparser::Operator::Switch { cont_type_index, tag_index } => wasm_encoder::Instruction::Switch { cont_type_index, tag_index },
    wasmparser::Operator::I64Add128 => wasm_encoder::Instruction::I64Add128,
    wasmparser::Operator::I64Sub128 => wasm_encoder::Instruction::I64Sub128,
    wasmparser::Operator::I64MulWideS => wasm_encoder::Instruction::I64MulWideS,
    wasmparser::Operator::I64MulWideU => wasm_encoder::Instruction::I64MulWideU,
    other => unimplemented!("{:?}", other),
  }
}

/// This test is a little bit tricky, but the goal is to be sure, that all `wasmparser` operators
/// are properly mapped into `wasm_encoder` instructions. As the [wasmparser::Operator] is non-exhaustive,
/// the easiest way is to compare the names of all mapped operators with all names defined in the enumeration.
/// This is done exactly in the following test.
#[test]
fn make_sure_all_operators_are_covered() {
  // Read the whole content of this file.
  let content = std::fs::read_to_string(file!()).unwrap();
  let mut operators = std::collections::BTreeSet::<String>::new();
  let mut collect_operators = false;
  for line in content.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
    // Iterate all lines until `map_operator` function definition, then start to collect operator names.
    if line.starts_with("pub fn map_operator") {
      collect_operators = true;
    }
    // Finish collecting operator name before the function definition ends.
    if line == "}" {
      collect_operators = false;
    }
    // Collect operator name.
    if collect_operators && line.starts_with("wasmparser::Operator::") {
      let operator = line.split_ascii_whitespace().next().unwrap()[22..].to_string();
      // Make sure the same name is used for operator and instruction.
      assert!(line.contains(&format!("wasm_encoder::Instruction::{}", operator)));
      operators.insert(operator);
    }
  }
  // Now the `operators` contains all operator names handled in the function `map_operator`.

  // Let's collect operator names from `wasmparser::Operator` enumeration.
  let mut enumerated_operators = std::collections::BTreeSet::<String>::new();

  macro_rules! collect_enumerated_operator {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $arg_type:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
      $( enumerated_operators.insert(stringify!($op).to_string()); )*
    }
  }
  wasmparser::for_each_operator!(collect_enumerated_operator);
  // Now the `enumerated_operators` contains all operator names defined in `wasmparser::Operator` enumeration.

  // Let's check, if all operators from the `wasmparser::Operator` enumeration are handled in the function `map_operator`.
  for enumerated_operator in &enumerated_operators {
    assert!(
      operators.contains(enumerated_operator),
      "Operator '{}' is not handled in `map_operator` function",
      enumerated_operator
    );
  }

  // Let's check, if all operators handled in the function `map_operator` are defined in `wasmparser::Operator` enumeration.
  for operator in &operators {
    assert!(
      enumerated_operators.contains(operator),
      "Operator '{}' is not defined in `wasmparser::Operator` enumeration",
      operator
    );
  }
}
