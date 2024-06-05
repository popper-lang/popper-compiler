use llvm_sys::analysis::LLVMVerifierFailureAction;



#[derive(Clone, Copy, PartialEq)]
pub enum FailureAction {
    AbortProcess,           // Abort the process and print to stderr
    PrintMessage,           // Print to stderr and return 1
    ReturnStatus            // Return 1 and print nothing
}


impl Into<LLVMVerifierFailureAction> for FailureAction {
    fn into(self) -> LLVMVerifierFailureAction {
        match self {
            FailureAction::AbortProcess => LLVMVerifierFailureAction::LLVMAbortProcessAction,
            FailureAction::PrintMessage => LLVMVerifierFailureAction::LLVMPrintMessageAction,
            FailureAction::ReturnStatus => LLVMVerifierFailureAction::LLVMReturnStatusAction
        }
    }
}

impl From<LLVMVerifierFailureAction> for FailureAction {
    fn from(action: LLVMVerifierFailureAction) -> Self {
        match action {
            LLVMVerifierFailureAction::LLVMAbortProcessAction => FailureAction::AbortProcess,
            LLVMVerifierFailureAction::LLVMPrintMessageAction => FailureAction::PrintMessage,
            LLVMVerifierFailureAction::LLVMReturnStatusAction => FailureAction::ReturnStatus
        }
    }
}
