/// A struct to keep track of metering properties.
#[derive(Default)]
pub struct Metering {
  pub remaining_points: u32,
}

impl Metering {
  pub fn remaining_points_type(&self) -> wasm_encoder::GlobalType {
    wasm_encoder::GlobalType {
      val_type: wasm_encoder::ValType::I64,
      mutable: true,
      shared: false,
    }
  }

  pub fn remaining_points_initial_value(&self) -> wasm_encoder::ConstExpr {
    wasm_encoder::ConstExpr::i64_const(0)
  }

  pub fn feed<'a>(&self, operator: wasmparser::Operator<'a>, accumulated_cost: i64) -> Vec<wasmparser::Operator<'a>> {
    if self.is_accounting_operator(&operator) {
      vec![
        wasmparser::Operator::GlobalGet {
          global_index: self.remaining_points,
        },
        wasmparser::Operator::I64Const { value: accumulated_cost },
        wasmparser::Operator::I64Sub,
        wasmparser::Operator::GlobalSet {
          global_index: self.remaining_points,
        },
        wasmparser::Operator::GlobalGet {
          global_index: self.remaining_points,
        },
        wasmparser::Operator::I64Const { value: 0 },
        wasmparser::Operator::I64LtS,
        wasmparser::Operator::If {
          blockty: wasmparser::BlockType::Empty,
        },
        wasmparser::Operator::Unreachable,
        wasmparser::Operator::End,
        operator,
      ]
    } else {
      vec![operator]
    }
  }

  pub fn cost(&self, operator: &wasmparser::Operator) -> i64 {
    match operator {
      wasmparser::Operator::End => 0,
      _ => 1,
    }
  }

  /// Returns `true` iff the given operator is an `accounting` operator.
  ///
  /// Before each `accounting` operator, there is an additional work
  /// to be done to track the metering points properly.
  fn is_accounting_operator(&self, operator: &wasmparser::Operator) -> bool {
    matches!(
      operator,
      wasmparser::Operator::Loop { .. } // loop headers are branch targets
            | wasmparser::Operator::End // block ends are branch targets
            | wasmparser::Operator::If { .. } // branch source, "if" can branch to else branch
            | wasmparser::Operator::Else // 'else' is the end of 'if' branch
            | wasmparser::Operator::Br { .. } // branch source
            | wasmparser::Operator::BrTable { .. } // branch source
            | wasmparser::Operator::BrIf { .. } // branch source
            | wasmparser::Operator::Call { .. } // function call ia a branch source
            | wasmparser::Operator::CallIndirect { .. } // function call is a branch source
            | wasmparser::Operator::Return // end of function is a branch source
            // exceptions proposal
            | wasmparser::Operator::Throw { .. } // branch source
            | wasmparser::Operator::ThrowRef // branch source
            | wasmparser::Operator::Rethrow { .. } // branch source
            | wasmparser::Operator::Delegate { .. } // branch source
            | wasmparser::Operator::Catch { .. } // branch target
            // tail_call proposal
            | wasmparser::Operator::ReturnCall { .. } // branch source
            | wasmparser::Operator::ReturnCallIndirect { .. } // branch source
            // gc proposal
            | wasmparser::Operator::BrOnCast { .. } // branch source
            | wasmparser::Operator::BrOnCastFail { .. } // branch source
            // function_references proposal
            | wasmparser::Operator::CallRef { .. } // branch source
            | wasmparser::Operator::ReturnCallRef { .. } // branch source
            | wasmparser::Operator::BrOnNull { .. } // branch source
            | wasmparser::Operator::BrOnNonNull { .. } // branch source
    )
  }
}
