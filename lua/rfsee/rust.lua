local ffi = require("ffi")

ffi.cdef([[
int hello_world(void)
]])

local lib = ffi.load("/Users/matth/projects/rfsee/crates/ffi/target/debug/libffi.dylib")

return lib
