local ffi = require("ffi")
local lfs = require("lfs")

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

local current_dir = lfs.currentdir();

local dylib = current_dir .. "/target/debug/libffi.so"
local lib = ffi.load(dylib)

local results = lib.search_terms("Hello")

if results == nil or results.error ~= 0 then
    error("Error searching")
end

print("Results: ", results)
