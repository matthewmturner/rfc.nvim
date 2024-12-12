local ffi = require("ffi")

ffi.cdef([[
typedef struct TermFrequencies* TermFrequenciesHandle;
typedef struct TfIdf* TfIdfHandle;

TermFrequenciesHandle tf_create();
void tf_insert_term(TermFrequenciesHandle, const char*, float);
// TermFrequenciesHandle extract_tf(const char*);

TfIdfHandle tf_idf_create();
void tf_idf_add_doc(TfIdfHandle, const char*, const char*);
void tf_idf_insert_doc_tfs(TfIdfHandle, const char*, TermFrequenciesHandle);
void tf_idf_finish(TfIdfHandle);
void tf_idf_save(TfIdfHandle, const char*);


]])

local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/release/libffi.dylib")

return lib
