//! This module contains paths to types and functions Clippy needs to know
//! about.
//!
//! Whenever possible, please consider diagnostic items over hardcoded paths.
//! See <https://github.com/rust-lang/rust-clippy/issues/5393> for more information.

pub const ANY_TRAIT: [&str; 3] = ["std", "any", "Any"];
pub const ARC_PTR_EQ: [&str; 4] = ["alloc", "sync", "Arc", "ptr_eq"];
pub const ASMUT_TRAIT: [&str; 3] = ["core", "convert", "AsMut"];
pub const ASREF_TRAIT: [&str; 3] = ["core", "convert", "AsRef"];
pub const BEGIN_PANIC: [&str; 3] = ["std", "panicking", "begin_panic"];
pub const BEGIN_PANIC_FMT: [&str; 3] = ["std", "panicking", "begin_panic_fmt"];
pub const BINARY_HEAP: [&str; 4] = ["alloc", "collections", "binary_heap", "BinaryHeap"];
pub const BORROW_TRAIT: [&str; 3] = ["core", "borrow", "Borrow"];
pub const BOX: [&str; 3] = ["alloc", "boxed", "Box"];
pub const BTREEMAP: [&str; 5] = ["alloc", "collections", "btree", "map", "BTreeMap"];
pub const BTREEMAP_ENTRY: [&str; 5] = ["alloc", "collections", "btree", "map", "Entry"];
pub const BTREESET: [&str; 5] = ["alloc", "collections", "btree", "set", "BTreeSet"];
pub const CLONE_TRAIT: [&str; 3] = ["core", "clone", "Clone"];
pub const CLONE_TRAIT_METHOD: [&str; 4] = ["core", "clone", "Clone", "clone"];
pub const CMP_MAX: [&str; 3] = ["core", "cmp", "max"];
pub const CMP_MIN: [&str; 3] = ["core", "cmp", "min"];
pub const COW: [&str; 3] = ["alloc", "borrow", "Cow"];
pub const CSTRING: [&str; 4] = ["std", "ffi", "c_str", "CString"];
pub const CSTRING_AS_C_STR: [&str; 5] = ["std", "ffi", "c_str", "CString", "as_c_str"];
pub const DEFAULT_TRAIT: [&str; 3] = ["core", "default", "Default"];
pub const DEFAULT_TRAIT_METHOD: [&str; 4] = ["core", "default", "Default", "default"];
pub const DEREF_MUT_TRAIT_METHOD: [&str; 5] = ["core", "ops", "deref", "DerefMut", "deref_mut"];
pub const DEREF_TRAIT_METHOD: [&str; 5] = ["core", "ops", "deref", "Deref", "deref"];
pub const DISPLAY_FMT_METHOD: [&str; 4] = ["core", "fmt", "Display", "fmt"];
pub const DISPLAY_TRAIT: [&str; 3] = ["core", "fmt", "Display"];
pub const DOUBLE_ENDED_ITERATOR: [&str; 4] = ["core", "iter", "traits", "DoubleEndedIterator"];
pub const DROP: [&str; 3] = ["core", "mem", "drop"];
pub const DROP_TRAIT: [&str; 4] = ["core", "ops", "drop", "Drop"];
pub const DURATION: [&str; 3] = ["core", "time", "Duration"];
pub const EARLY_CONTEXT: [&str; 4] = ["rustc", "lint", "context", "EarlyContext"];
pub const EXIT: [&str; 3] = ["std", "process", "exit"];
pub const F32_EPSILON: [&str; 2] = ["f32", "EPSILON"];
pub const F64_EPSILON: [&str; 2] = ["f64", "EPSILON"];
pub const FILE: [&str; 3] = ["std", "fs", "File"];
pub const FILE_TYPE: [&str; 3] = ["std", "fs", "FileType"];
pub const FMT_ARGUMENTS_NEW_V1: [&str; 4] = ["core", "fmt", "Arguments", "new_v1"];
pub const FMT_ARGUMENTS_NEW_V1_FORMATTED: [&str; 4] = ["core", "fmt", "Arguments", "new_v1_formatted"];
pub const FMT_ARGUMENTV1_NEW: [&str; 4] = ["core", "fmt", "ArgumentV1", "new"];
pub const FROM_FROM: [&str; 4] = ["core", "convert", "From", "from"];
pub const FROM_TRAIT: [&str; 3] = ["core", "convert", "From"];
pub const FUTURE_FROM_GENERATOR: [&str; 3] = ["core", "future", "from_generator"];
pub const HASH: [&str; 2] = ["hash", "Hash"];
pub const HASHMAP: [&str; 5] = ["std", "collections", "hash", "map", "HashMap"];
pub const HASHMAP_ENTRY: [&str; 5] = ["std", "collections", "hash", "map", "Entry"];
pub const HASHSET: [&str; 5] = ["std", "collections", "hash", "set", "HashSet"];
pub const INDEX: [&str; 3] = ["core", "ops", "Index"];
pub const INDEX_MUT: [&str; 3] = ["core", "ops", "IndexMut"];
pub const INTO: [&str; 3] = ["core", "convert", "Into"];
pub const INTO_ITERATOR: [&str; 5] = ["core", "iter", "traits", "collect", "IntoIterator"];
pub const IO_READ: [&str; 3] = ["std", "io", "Read"];
pub const IO_WRITE: [&str; 3] = ["std", "io", "Write"];
pub const ITERATOR: [&str; 5] = ["core", "iter", "traits", "iterator", "Iterator"];
pub const LATE_CONTEXT: [&str; 4] = ["rustc", "lint", "context", "LateContext"];
pub const LINKED_LIST: [&str; 4] = ["alloc", "collections", "linked_list", "LinkedList"];
pub const LINT: [&str; 3] = ["rustc_session", "lint", "Lint"];
pub const MEM_DISCRIMINANT: [&str; 3] = ["core", "mem", "discriminant"];
pub const MEM_FORGET: [&str; 3] = ["core", "mem", "forget"];
pub const MEM_MANUALLY_DROP: [&str; 4] = ["core", "mem", "manually_drop", "ManuallyDrop"];
pub const MEM_MAYBEUNINIT: [&str; 4] = ["core", "mem", "maybe_uninit", "MaybeUninit"];
pub const MEM_MAYBEUNINIT_UNINIT: [&str; 5] = ["core", "mem", "maybe_uninit", "MaybeUninit", "uninit"];
pub const MEM_REPLACE: [&str; 3] = ["core", "mem", "replace"];
pub const MUTEX_GUARD: [&str; 4] = ["std", "sync", "mutex", "MutexGuard"];
pub const OPEN_OPTIONS: [&str; 3] = ["std", "fs", "OpenOptions"];
pub const OPS_MODULE: [&str; 2] = ["core", "ops"];
pub const OPTION: [&str; 3] = ["core", "option", "Option"];
pub const OPTION_NONE: [&str; 4] = ["core", "option", "Option", "None"];
pub const OPTION_SOME: [&str; 4] = ["core", "option", "Option", "Some"];
pub const ORD: [&str; 3] = ["core", "cmp", "Ord"];
pub const OS_STRING: [&str; 4] = ["std", "ffi", "os_str", "OsString"];
pub const OS_STRING_AS_OS_STR: [&str; 5] = ["std", "ffi", "os_str", "OsString", "as_os_str"];
pub const OS_STR_TO_OS_STRING: [&str; 5] = ["std", "ffi", "os_str", "OsStr", "to_os_string"];
pub const PARKING_LOT_MUTEX_GUARD: [&str; 2] = ["parking_lot", "MutexGuard"];
pub const PARKING_LOT_RWLOCK_READ_GUARD: [&str; 2] = ["parking_lot", "RwLockReadGuard"];
pub const PARKING_LOT_RWLOCK_WRITE_GUARD: [&str; 2] = ["parking_lot", "RwLockWriteGuard"];
pub const PATH: [&str; 3] = ["std", "path", "Path"];
pub const PATH_BUF: [&str; 3] = ["std", "path", "PathBuf"];
pub const PATH_BUF_AS_PATH: [&str; 4] = ["std", "path", "PathBuf", "as_path"];
pub const PATH_TO_PATH_BUF: [&str; 4] = ["std", "path", "Path", "to_path_buf"];
pub const POLL: [&str; 4] = ["core", "task", "poll", "Poll"];
pub const PTR_EQ: [&str; 3] = ["core", "ptr", "eq"];
pub const PTR_NULL: [&str; 2] = ["ptr", "null"];
pub const PTR_NULL_MUT: [&str; 2] = ["ptr", "null_mut"];
pub const PUSH_STR: [&str; 4] = ["alloc", "string", "String", "push_str"];
pub const RANGE_ARGUMENT_TRAIT: [&str; 3] = ["core", "ops", "RangeBounds"];
pub const RC: [&str; 3] = ["alloc", "rc", "Rc"];
pub const RC_PTR_EQ: [&str; 4] = ["alloc", "rc", "Rc", "ptr_eq"];
pub const RECEIVER: [&str; 4] = ["std", "sync", "mpsc", "Receiver"];
pub const REGEX_BUILDER_NEW: [&str; 5] = ["regex", "re_builder", "unicode", "RegexBuilder", "new"];
pub const REGEX_BYTES_BUILDER_NEW: [&str; 5] = ["regex", "re_builder", "bytes", "RegexBuilder", "new"];
pub const REGEX_BYTES_NEW: [&str; 4] = ["regex", "re_bytes", "Regex", "new"];
pub const REGEX_BYTES_SET_NEW: [&str; 5] = ["regex", "re_set", "bytes", "RegexSet", "new"];
pub const REGEX_NEW: [&str; 4] = ["regex", "re_unicode", "Regex", "new"];
pub const REGEX_SET_NEW: [&str; 5] = ["regex", "re_set", "unicode", "RegexSet", "new"];
pub const REPEAT: [&str; 3] = ["core", "iter", "repeat"];
pub const RESULT: [&str; 3] = ["core", "result", "Result"];
pub const RESULT_ERR: [&str; 4] = ["core", "result", "Result", "Err"];
pub const RESULT_OK: [&str; 4] = ["core", "result", "Result", "Ok"];
pub const RWLOCK_READ_GUARD: [&str; 4] = ["std", "sync", "rwlock", "RwLockReadGuard"];
pub const RWLOCK_WRITE_GUARD: [&str; 4] = ["std", "sync", "rwlock", "RwLockWriteGuard"];
pub const SERDE_DESERIALIZE: [&str; 2] = ["_serde", "Deserialize"];
pub const SERDE_DE_VISITOR: [&str; 3] = ["serde", "de", "Visitor"];
pub const SLICE_INTO_VEC: [&str; 4] = ["alloc", "slice", "<impl [T]>", "into_vec"];
pub const SLICE_ITER: [&str; 4] = ["core", "slice", "iter", "Iter"];
pub const STDERR: [&str; 4] = ["std", "io", "stdio", "stderr"];
pub const STDOUT: [&str; 4] = ["std", "io", "stdio", "stdout"];
pub const STD_CONVERT_IDENTITY: [&str; 3] = ["std", "convert", "identity"];
pub const STD_FS_CREATE_DIR: [&str; 3] = ["std", "fs", "create_dir"];
pub const STD_MEM_TRANSMUTE: [&str; 3] = ["std", "mem", "transmute"];
pub const STD_PTR_NULL: [&str; 3] = ["std", "ptr", "null"];
pub const STRING_AS_MUT_STR: [&str; 4] = ["alloc", "string", "String", "as_mut_str"];
pub const STRING_AS_STR: [&str; 4] = ["alloc", "string", "String", "as_str"];
pub const SYNTAX_CONTEXT: [&str; 3] = ["rustc_span", "hygiene", "SyntaxContext"];
pub const TO_OWNED: [&str; 3] = ["alloc", "borrow", "ToOwned"];
pub const TO_OWNED_METHOD: [&str; 4] = ["alloc", "borrow", "ToOwned", "to_owned"];
pub const TO_STRING: [&str; 3] = ["alloc", "string", "ToString"];
pub const TO_STRING_METHOD: [&str; 4] = ["alloc", "string", "ToString", "to_string"];
pub const TRANSMUTE: [&str; 4] = ["core", "intrinsics", "", "transmute"];
pub const TRY_FROM: [&str; 4] = ["core", "convert", "TryFrom", "try_from"];
pub const TRY_INTO_TRAIT: [&str; 3] = ["core", "convert", "TryInto"];
pub const VEC: [&str; 3] = ["alloc", "vec", "Vec"];
pub const VEC_AS_MUT_SLICE: [&str; 4] = ["alloc", "vec", "Vec", "as_mut_slice"];
pub const VEC_AS_SLICE: [&str; 4] = ["alloc", "vec", "Vec", "as_slice"];
pub const VEC_DEQUE: [&str; 4] = ["alloc", "collections", "vec_deque", "VecDeque"];
pub const VEC_FROM_ELEM: [&str; 3] = ["alloc", "vec", "from_elem"];
pub const VEC_NEW: [&str; 4] = ["alloc", "vec", "Vec", "new"];
pub const VEC_RESIZE: [&str; 4] = ["alloc", "vec", "Vec", "resize"];
pub const WEAK_ARC: [&str; 3] = ["alloc", "sync", "Weak"];
pub const WEAK_RC: [&str; 3] = ["alloc", "rc", "Weak"];
