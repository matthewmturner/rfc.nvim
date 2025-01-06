local ffi = require("ffi")

ffi.cdef([[
typedef void (*progress_callback_t)(double progress);
void build_index(progress_callback_t progress_cb);

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
local lib_extension = (jit.os == "OSX") and ".dylib" or ".so"
local dylib = script_dir .. "/artifacts/libffi" .. lib_extension

local lib = ffi.load(dylib)

return lib
