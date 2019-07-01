use wasm_bindgen::prelude::*;
use std::io::Cursor;
use web_sys::console;
use zokrates_core::proof_system::{self, ProofSystem};
use zokrates_core::ir;
use zokrates_field::field::FieldPrime;
use bincode::{deserialize_from, Infinite};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    let prog_string = include_bytes!("../static/out").to_vec();
    let key_string = include_bytes!("../static/proving.key");
	
	let mut prog_cursor = Cursor::new(&prog_string);

    let program: ir::Prog<FieldPrime> =
                deserialize_from(&mut prog_cursor, Infinite).unwrap();
    let witness = program.execute(&vec![FieldPrime::from(0), FieldPrime::from(0)]).unwrap();

    console::log_1(&JsValue::from_str(&format!("{}", program)));
    console::time();

    let proof = proof_system::G16{}.generate_proof_wasm(program, witness, key_string); 

    console::time_end();

    console::log_1(&JsValue::from_str(&format!("{}", proof)));

    Ok(())
}
