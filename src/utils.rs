//! Utility data structures and functions.

use alloc::collections::btree_set::BTreeSet;

use parity_wasm::elements;

/// Gather all the imported function ids.
pub fn imported_function_ids(module: &elements::Module) -> BTreeSet<u32> {
	let func_imports = module.import_count(elements::ImportCountType::Function);

	(0..module.functions_space())
		.filter_map(|func_idx| if func_idx < func_imports { Some(func_idx as u32) } else { None })
		.collect()
}
