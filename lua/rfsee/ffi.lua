local ffi = require("ffi")

ffi.cdef([[

struct RfcSearchResult {
    const char* url;
    const char* title;
};

struct RfcSearchResults {
    int len;
    const struct RfcSearchResult* rfcs;
    int error;
};

// The function returning a pointer to RfcSearchResults
struct RfcSearchResults* search_terms(const char* terms);

]])

local script_dir = vim.fn.expand("<sfile>:p:h:h")
local dylib = script_dir .. "/crates/ffi/target/release/libffi.dylib"

local lib = ffi.load(dylib)
-- local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/release/libffi.dylib")

return lib
