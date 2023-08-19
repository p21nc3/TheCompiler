use crate::parser::NodeExit;

pub(crate) struct Generator {
    root: NodeExit,
}

impl Generator {
    pub fn new(root: NodeExit) -> Self {
        Self { root }
    }

    pub fn generate(&self) -> String {
        let mut output = String::new();
        write!(
            output,
            "global _start\n_start:\n\
             mov rax, 60\n\
             mov rdi, {}\n\
             syscall",
            self.root.expr.int_lit.value.value()
        )
        .expect("Error writing to output string");
        output
    }
}
