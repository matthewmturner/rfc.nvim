local ffi = require("ffi")

ffi.cdef([[
typedef struct {
    bool error;
} SaveResult;

int hello_world(void);
SaveResult save_json(void);
SaveResult save_input_number_as_json(int);
SaveResult save_input_number_as_json_to_custom_path(int, const char*);
]])

local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/debug/libffi.dylib")

return lib
