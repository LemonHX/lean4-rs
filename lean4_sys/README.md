# Lean4 sys
[![crates.io](https://img.shields.io/crates/v/lean4_sys)](https://crates.io/crates/lean4_sys)
[![docs.rs](https://img.shields.io/docsrs/lean4_sys)](https://docs.rs/crate/lean4_sys/latest)

very low level bindings to the Lean4 runtime

## Usage

first ensure you install `lean4` with `elan` and `lake`

and ensure there is at least one C compiler in your path

## and how do I use Rust libraries in Lean4?
add following to your `lakefile.lean`:

```lean
def cargoBuildRelease (name : String) (tomlFileDir : FilePath) (moreArgs : Array String := #[]) : BuildM Unit := do
  let manifestPath  :=  (tomlFileDir / "Cargo.toml").toString
  logStep s!"Compiling {name} with manifest path {manifestPath}"
  
  proc {
    cmd := "cargo"
    args := #["build", "--release", "--manifest-path", manifestPath] ++ moreArgs
  }


def buildCargo (name : String) (tomlFileDir : FilePath) (moreArgs : Array String := #[]) : SchedulerM (BuildJob FilePath) := do
  let file := tomlFileDir / "target" / "release" / (nameToStaticLib name)
  BuildJob.nil.bindSync fun _ _ => do
    let trace ← buildFileUnlessUpToDate file (← (pure BuildTrace.nil)) <| (cargoBuildRelease name tomlFileDir moreArgs)
    return (file, trace)
```

then you can use like following:

```lean
target lean_test_rs (pkg : Package) : FilePath := do
  buildCargo "lean_test" (pkg.dir / "lean_test")


extern_lib lean_test (pkg : Package) := do
  fetch <| pkg.target ``lean_test_rs
```

then you can enjoy Rust in Lean4

```lean
@[extern "rust_hello"]
opaque hello : BaseIO Unit
```

```rust
// in your cargo project
use lean4_sys::*;

#[no_mangle]
pub extern "C" fn lean_rust_hello(_: lean_obj_arg) -> lean_obj_res {
    unsafe { 
        println!("Hello from Rust🦀!");
        lean_io_result_mk_ok(lean_box(0)) 
    }
}
```