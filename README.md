# icu4x_compile_sample

See: https://github.com/unicode-org/icu4x/issues/5230

```
/path/to/baked_data_slow/demo$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
     Removed 0 files
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
warning: /path/to/baked_data_slow/experimental_data/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2021
   Compiling icu_experimental_data v0.1.0 (/path/to/baked_data_slow/experimental_data)
time:   0.000; rss:   47MB ->   47MB (   +1MB)	parse_crate
time:   0.000; rss:   48MB ->   48MB (   +0MB)	incr_comp_garbage_collect_session_directories
time:   0.000; rss:   48MB ->   49MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   50MB ->   51MB (   +1MB)	crate_injection
time:   0.003; rss:   51MB ->   65MB (  +14MB)	expand_crate
time:   0.003; rss:   51MB ->   65MB (  +14MB)	macro_expand_crate
time:   0.000; rss:   65MB ->   65MB (   +0MB)	finalize_imports
time:   0.000; rss:   65MB ->   67MB (   +2MB)	finalize_macro_resolutions
time:   0.000; rss:   67MB ->   68MB (   +2MB)	late_resolve_crate
time:   0.000; rss:   68MB ->   68MB (   +0MB)	resolve_postprocess
time:   0.001; rss:   65MB ->   68MB (   +4MB)	resolve_crate
time:   0.000; rss:   68MB ->   69MB (   +0MB)	write_dep_info
time:   0.000; rss:   69MB ->   69MB (   +0MB)	complete_gated_feature_checking
time:   0.000; rss:   69MB ->   70MB (   +2MB)	looking_for_entry_point
time:   0.000; rss:   71MB ->   71MB (   +0MB)	unused_lib_feature_checking
time:   0.001; rss:   69MB ->   71MB (   +3MB)	misc_checking_1
time:   0.000; rss:   71MB ->   75MB (   +4MB)	coherence_checking
time:   0.002; rss:   71MB ->   84MB (  +12MB)	type_check_crate
time:   0.001; rss:   84MB ->   89MB (   +6MB)	MIR_borrow_checking
time:   0.000; rss:   89MB ->   91MB (   +2MB)	MIR_effect_checking
time:   0.001; rss:   91MB ->   91MB (   +0MB)	misc_checking_3
time:   0.002; rss:   91MB ->   97MB (   +6MB)	monomorphization_collector_graph_walk
time:   0.000; rss:   97MB ->   98MB (   +0MB)	partition_and_assert_distinct_symbols
time:   0.000; rss:  100MB ->  103MB (   +3MB)	write_allocator_module
time:   0.017; rss:  117MB ->  121MB (   +4MB)	LLVM_passes
time:   0.003; rss:  107MB ->  121MB (  +14MB)	codegen_to_LLVM_IR
time:   0.024; rss:   91MB ->  121MB (  +29MB)	codegen_crate
time:   0.000; rss:  121MB ->  121MB (   +0MB)	encode_query_results
time:   0.000; rss:  121MB ->  121MB (   +0MB)	incr_comp_serialize_result_cache
time:   0.001; rss:  121MB ->  121MB (   +0MB)	incr_comp_persist_result_cache
time:   0.001; rss:  121MB ->  121MB (   +0MB)	serialize_dep_graph
time:   0.002; rss:  121MB ->   96MB (  -25MB)	free_global_ctxt
time:   0.033; rss:   97MB ->   97MB (   +0MB)	run_linker
time:   0.036; rss:   96MB ->   97MB (   +1MB)	link_binary
time:   0.036; rss:   96MB ->   97MB (   +1MB)	link_crate
time:   0.037; rss:   96MB ->   97MB (   +1MB)	link
time:   0.080; rss:   32MB ->   92MB (  +61MB)	total
time:   0.000; rss:   47MB ->   47MB (   +0MB)	parse_crate
time:   0.000; rss:   48MB ->   48MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   50MB ->   50MB (   +1MB)	crate_injection
time:   0.729; rss:   50MB ->  905MB ( +854MB)	expand_crate
time:   0.729; rss:   50MB ->  905MB ( +854MB)	macro_expand_crate
time:   0.000; rss:  905MB ->  905MB (   +0MB)	finalize_imports
time:   0.000; rss:  905MB ->  907MB (   +1MB)	finalize_macro_resolutions
time:   0.000; rss:  907MB ->  907MB (   +0MB)	late_resolve_crate
time:   0.000; rss:  907MB ->  907MB (   +0MB)	resolve_postprocess
time:   0.001; rss:  905MB ->  907MB (   +2MB)	resolve_crate
time:   0.001; rss:  828MB ->  828MB (   +0MB)	write_dep_info
time:   0.000; rss:  828MB ->  829MB (   +0MB)	complete_gated_feature_checking
time:   0.158; rss:  828MB ->  829MB (   +1MB)	looking_for_derive_registrar
time:   0.158; rss:  828MB ->  830MB (   +2MB)	misc_checking_1
time:   0.000; rss:  830MB ->  830MB (   +0MB)	coherence_checking
time:   0.000; rss:  830MB ->  830MB (   +1MB)	type_check_crate
time:   0.000; rss:  831MB ->  831MB (   +1MB)	module_lints
time:   0.001; rss:  831MB ->  831MB (   +1MB)	lint_checking
time:   0.001; rss:  830MB ->  831MB (   +1MB)	misc_checking_3
time:   0.509; rss:  831MB ->  732MB ( -100MB)	generate_crate_metadata
time:   0.000; rss:  732MB ->  748MB (  +16MB)	codegen_to_LLVM_IR
time:   0.002; rss:  742MB ->  748MB (   +7MB)	LLVM_passes
time:   0.003; rss:  732MB ->  748MB (  +16MB)	codegen_crate
time:   0.000; rss:  748MB ->  748MB (   +0MB)	incr_comp_persist_result_cache
time:   0.001; rss:  748MB ->  748MB (   +0MB)	serialize_dep_graph
time:   0.026; rss:  748MB ->  710MB (  -38MB)	free_global_ctxt
time:   0.112; rss:  710MB ->  772MB (  +63MB)	link_rlib
time:   0.125; rss:  710MB ->  772MB (  +63MB)	link_binary
time:   0.129; rss:  710MB ->  710MB (   +0MB)	link_crate
time:   0.130; rss:  710MB ->  710MB (   +0MB)	link
time:   1.628; rss:   32MB ->   93MB (  +61MB)	total
   Compiling demo v0.1.0 (/path/to/baked_data_slow/demo)
time:   0.001; rss:   46MB ->   48MB (   +1MB)	parse_crate
time:   0.000; rss:   48MB ->   49MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   50MB ->   51MB (   +0MB)	crate_injection
time:   0.375; rss:   51MB ->  530MB ( +478MB)	expand_crate
time:   0.375; rss:   51MB ->  530MB ( +479MB)	macro_expand_crate
time:   0.000; rss:  530MB ->  530MB (   +0MB)	maybe_building_test_harness
time:   0.010; rss:  530MB ->  530MB (   +0MB)	AST_validation
time:   0.000; rss:  530MB ->  530MB (   +0MB)	finalize_imports
time:   0.000; rss:  530MB ->  530MB (   +1MB)	finalize_macro_resolutions
time:   0.167; rss:  530MB ->  572MB (  +41MB)	late_resolve_crate
time:   0.010; rss:  572MB ->  572MB (   +0MB)	resolve_check_unused
time:   0.020; rss:  572MB ->  572MB (   +0MB)	resolve_postprocess
time:   0.198; rss:  530MB ->  572MB (  +42MB)	resolve_crate
time:   0.011; rss:  543MB ->  543MB (   +0MB)	write_dep_info
time:   0.009; rss:  543MB ->  543MB (   +0MB)	complete_gated_feature_checking
time:   0.032; rss:  703MB ->  627MB (  -76MB)	drop_ast
time:   0.585; rss:  543MB ->  592MB (  +49MB)	looking_for_derive_registrar
time:   0.685; rss:  543MB ->  593MB (  +50MB)	misc_checking_1
time:   0.022; rss:  593MB ->  603MB (  +10MB)	coherence_checking
time:   1.577; rss:  593MB ->  823MB ( +230MB)	type_check_crate
time:  18.980; rss:  823MB -> 1068MB ( +245MB)	MIR_borrow_checking
time:   0.626; rss: 1068MB -> 1052MB (  -15MB)	MIR_effect_checking
time:   0.040; rss: 1053MB -> 1053MB (   +0MB)	module_lints
time:   0.040; rss: 1053MB -> 1053MB (   +0MB)	lint_checking
time:   0.095; rss: 1053MB -> 1053MB (   +0MB)	privacy_checking_modules
time:   0.018; rss: 1053MB -> 1053MB (   +0MB)	check_lint_expectations
time:   0.184; rss: 1052MB -> 1053MB (   +1MB)	misc_checking_3
time:   0.002; rss: 1054MB -> 1055MB (   +0MB)	monomorphization_collector_graph_walk
time:   0.073; rss: 1053MB -> 1055MB (   +1MB)	generate_crate_metadata
time:   0.028; rss: 1055MB -> 1076MB (  +21MB)	codegen_to_LLVM_IR
time:   0.102; rss: 1066MB -> 1075MB (  +10MB)	LLVM_passes
time:   0.129; rss: 1055MB -> 1075MB (  +21MB)	codegen_crate
time:   0.128; rss: 1075MB -> 1075MB (   +0MB)	encode_query_results
time:   0.134; rss: 1075MB -> 1075MB (   +0MB)	incr_comp_serialize_result_cache
time:   0.134; rss: 1075MB -> 1075MB (   +0MB)	incr_comp_persist_result_cache
time:   0.135; rss: 1075MB -> 1075MB (   +0MB)	serialize_dep_graph
time:   0.034; rss: 1076MB ->  599MB ( -477MB)	free_global_ctxt
time:   0.031; rss:  599MB ->  614MB (  +14MB)	link_rlib
time:   0.032; rss:  599MB ->  614MB (  +14MB)	link_binary
time:   0.033; rss:  599MB ->  599MB (   +0MB)	link_crate
time:   0.034; rss:  599MB ->  599MB (   +0MB)	link
time:  23.099; rss:   32MB ->  191MB ( +159MB)	total
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 24.94s
	Command being timed: "cargo +nightly build -j1 --all-features"
	User time (seconds): 16.38
	System time (seconds): 8.58
	Percent of CPU this job got: 100%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:24.96
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 14819620
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 17
	Minor (reclaiming a frame) page faults: 3672602
	Voluntary context switches: 847
	Involuntary context switches: 195
	Swaps: 0
	File system inputs: 0
	File system outputs: 554672
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
```

### Copyright & Licenses

Copyright © 2023 Unicode, Inc. Unicode and the Unicode Logo are registered trademarks of Unicode, Inc. in the United States and other countries.

The project is released under [LICENSE](./LICENSE).

A CLA is required to contribute to this project - please refer to the [CONTRIBUTING.md](https://github.com/unicode-org/.github/blob/main/.github/CONTRIBUTING.md) file (or start a Pull Request) for more information.
