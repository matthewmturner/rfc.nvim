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
print("Lib path: ", dylib)
local lib = ffi.load(dylib)

local results = lib.search_terms("Hello")
print("Results: ", results.error)

-- Convert RFC results into lines
for i = 0, results.len - 1 do
    local rfc = results.rfcs[i]
    -- Convert C strings to Lua strings and remove any newlines.
    local title = ffi.string(rfc.title):gsub("\n", " ")
    print("Title: ", title)
end


-- if results == nil or results.error ~= 0 then
--     error("Error searching: ", results.error)
-- end
