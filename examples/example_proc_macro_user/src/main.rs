use example_proc_macro::DebuggableProcMacro;

#[derive(DebuggableProcMacro)]
struct _StructA;

#[derive(DebuggableProcMacro)]
struct _StructB;

fn main() {
  panic!("The example is the proc_macro itself, see examples/example_proc_macro");
}
