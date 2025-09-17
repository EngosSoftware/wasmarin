use wasmparser::Operator;

pub fn metering(operator: Operator) -> Vec<Operator> {
  if is_accounting_operator(&operator) {
    vec![Operator::LocalTee { local_index: 0 }, operator]
  } else {
    vec![operator]
  }
}

pub fn metering_cost(_operator: &Operator) -> u64 {
  1
}

/// Returns `true` iff the given operator is an `accounting` operator.
///
/// Before each `accounting` operator, there is an additional work
/// to be done to track the metering points properly.
fn is_accounting_operator(operator: &Operator) -> bool {
  matches!(
    operator,
    Operator::Loop { .. } // loop headers are branch targets
            | Operator::End // block ends are branch targets
            | Operator::If { .. } // branch source, "if" can branch to else branch
            | Operator::Else // 'else' is the end of 'if' branch
            | Operator::Br { .. } // branch source
            | Operator::BrTable { .. } // branch source
            | Operator::BrIf { .. } // branch source
            | Operator::Call { .. } // function call ia a branch source
            | Operator::CallIndirect { .. } // function call is a branch source
            | Operator::Return // end of function is a branch source
            // exceptions proposal
            | Operator::Throw { .. } // branch source
            | Operator::ThrowRef // branch source
            | Operator::Rethrow { .. } // branch source
            | Operator::Delegate { .. } // branch source
            | Operator::Catch { .. } // branch target
            // tail_call proposal
            | Operator::ReturnCall { .. } // branch source
            | Operator::ReturnCallIndirect { .. } // branch source
            // gc proposal
            | Operator::BrOnCast { .. } // branch source
            | Operator::BrOnCastFail { .. } // branch source
            // function_references proposal
            | Operator::CallRef { .. } // branch source
            | Operator::ReturnCallRef { .. } // branch source
            | Operator::BrOnNull { .. } // branch source
            | Operator::BrOnNonNull { .. } // branch source
  )
}
