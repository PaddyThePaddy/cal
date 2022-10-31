pub type UintType = u128;

pub trait ToEvalexprError {
  fn to_eval_error(&self) -> evalexpr::EvalexprError;
}

impl<T> ToEvalexprError for T
where
  T: std::fmt::Display,
{
  fn to_eval_error(&self) -> evalexpr::EvalexprError {
    evalexpr::EvalexprError::CustomMessage(format!("{}", self))
  }
}

pub trait ToEvalexprResult<T> {
  fn to_eval_result(self) -> evalexpr::EvalexprResult<T>;
}

impl<T, E> ToEvalexprResult<T> for Result<T, E>
where
  E: std::fmt::Display,
{
  fn to_eval_result(self) -> evalexpr::EvalexprResult<T> {
    self.map_err(|e| e.to_eval_error())
  }
}
