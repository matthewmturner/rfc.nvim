local ffi = require("ffi")

ffi.cdef([[
typedef struct {
    bool error;
} SaveResult;

typedef struct TermFrequencies* TermFrequenciesHandle;
typedef struct TfIdf* TfIdfHandle;

SaveResult save_json(void);
SaveResult save_input_number_as_json(int);
SaveResult save_input_number_as_json_to_custom_path(int, const char*);

TermFrequenciesHandle create_term_freqs();
void insert_term_freqs(TermFrequenciesHandle, const char*, float);

TfIdfHandle create_tf_idf();
void insert_tf_idf(TfIdfHandle, const char*, TermFrequenciesHandle);
void save_tf_idf(TfIdfHandle, const char*);


]])

local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/debug/libffi.dylib")

return lib
