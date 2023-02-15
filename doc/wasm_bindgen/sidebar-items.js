window.SIDEBAR_ITEMS = {"fn":[["externref_heap_live_count","Get the count of live `externref`s / `JsValue`s in `wasm-bindgen`’s heap."],["function_table","Returns a handle to this wasm instance’s `WebAssembly.Table` which is the indirect function table used by Rust"],["intern","Interns Rust strings so that it’s much faster to send them to JS."],["memory","Returns a handle to this wasm instance’s `WebAssembly.Memory`"],["throw_str","Throws a JS exception."],["throw_val","Rethrow a JS exception"],["unintern","Removes a Rust string from the intern cache."]],"macro":[["link_to","This macro takes a JS module as input and returns a URL that can be used to access it at runtime."]],"mod":[["closure","Support for long-lived closures in `wasm-bindgen`"],["convert","This is mostly an internal module, no stability guarantees are provided. Use at your own risk."],["prelude","A module which is typically glob imported."]],"struct":[["Clamped","A wrapper type around slices and vectors for binding the `Uint8ClampedArray` array in JS."],["JsError","Convenience type for use on exported `fn() -> Result<T, JsError>` functions, where you wish to throw a JavaScript `Error` object."],["JsStatic","Wrapper type for imported statics."],["JsValue","Representation of an object owned by JS."]],"trait":[["JsCast","A trait for checked and unchecked casting between JS types."],["UnwrapThrowExt","An extension trait for `Option<T>` and `Result<T, E>` for unwrapping the `T` value, or throwing a JS error if it is not available."]]};