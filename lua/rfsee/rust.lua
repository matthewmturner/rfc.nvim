local ffi = require("ffi")

ffi.cdef([[
typedef struct {
    bool error;
} SaveResult;

typedef struct TermFrequencies* TermFrequenciesHandle;
typedef struct TfIdfBuilder* TfIdfBuilderHandle;
typedef struct TfIdf* TfIdfHandle;

// SaveResult save_json(void);
// SaveResult save_input_number_as_json(int);
// SaveResult save_input_number_as_json_to_custom_path(int, const char*);

TermFrequenciesHandle tf_create();
void tf_insert_term(TermFrequenciesHandle, const char*, float);

TfIdfBuilderHandle tf_idf_builder_create();
void tf_idf_builder_insert_doc_tfs(TfIdfBuilderHandle, const char*, TermFrequenciesHandle);
TfIdfHandle tf_idf_builder_finish(TfIdfBuilderHandle);

// void save_tf_idf(TfIdfHandle, const char*);


]])

local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/debug/libffi.dylib")

return lib
