use crate::mappings::map_operator;

/// Exported name of the global variable for keeping track of remaining points.
const EXPORT_NAME_REMAINING_POINTS: &str = "wasmarin_metering_remaining_points";

/// Metering properties.
#[derive(Default)]
pub struct Metering {
  /// Enables metering functionality.
  enabled: bool,
  /// Index of a global variable storing remaining points.
  remaining_points_global_index: u32,
}

impl Metering {
  /// Creates a new [Metering] instance.
  pub fn new(enabled: bool) -> Self {
    Self {
      enabled,
      remaining_points_global_index: 0,
    }
  }

  /// Adds a global variable to keep track of remaining points.
  pub fn update_global_section(&mut self, global_section: &mut wasm_encoder::GlobalSection) {
    if self.enabled {
      self.remaining_points_global_index = global_section.len();
      global_section.global(
        wasm_encoder::GlobalType {
          val_type: wasm_encoder::ValType::I64,
          mutable: true,
          shared: false,
        },
        &wasm_encoder::ConstExpr::i64_const(0),
      );
    }
  }

  /// Adds and export name for the global variable that keeps track of remaining points.
  pub fn update_export_section(&mut self, export_section: &mut wasm_encoder::ExportSection) {
    if self.enabled {
      export_section.export(EXPORT_NAME_REMAINING_POINTS, wasm_encoder::ExportKind::Global, self.remaining_points_global_index);
    }
  }

  /// Updates function's operator with metering code.
  pub fn update_function(&mut self, function: &mut wasm_encoder::Function, operators: Vec<wasmparser::Operator>) {
    if self.enabled {
      let mut accumulated_cost = 0;
      for operator in operators {
        accumulated_cost += self.cost(&operator);
        for op in self.feed(operator, accumulated_cost) {
          function.instruction(&map_operator(op));
        }
      }
    } else {
      for operator in operators {
        function.instruction(&map_operator(operator));
      }
    }
  }

  fn feed<'a>(&self, operator: wasmparser::Operator<'a>, accumulated_cost: i64) -> Vec<wasmparser::Operator<'a>> {
    if self.is_accounting_operator(&operator) {
      vec![
        wasmparser::Operator::GlobalGet {
          global_index: self.remaining_points_global_index,
        },
        wasmparser::Operator::I64Const { value: accumulated_cost },
        wasmparser::Operator::I64Sub,
        wasmparser::Operator::GlobalSet {
          global_index: self.remaining_points_global_index,
        },
        wasmparser::Operator::GlobalGet {
          global_index: self.remaining_points_global_index,
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

  fn cost(&self, operator: &wasmparser::Operator) -> i64 {
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
