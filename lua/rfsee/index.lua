local lib      = require("rfsee.ffi")
local window   = require("rfsee.window")
local ffi      = require("ffi")
local curl     = require("plenary.curl")

local M        = {}

RFC_INDEX_URL  = "https://www.ietf.org/rfc/rfc-index.txt"
RFC_DELIMITTER = "\n\n"

function M.search_terms(terms)
    local results = lib.search_terms(terms)

    -- Check for errors
    if results == nil or results.error ~= 0 then
        print("Error occurred during search")
        return
    end

    -- Convert RFC results into lines
    local lines = {}
    for i = 0, results.len - 1 do
        local rfc = results.rfcs[i]
        -- Convert C strings to Lua strings and remove any newlines.
        local title = ffi.string(rfc.title):gsub("\n", " ")
        local url = ffi.string(rfc.url):gsub("\n", " ")
        table.insert(lines, title .. " - " .. url)
    end

    -- Create a new scratch buffer
    local results_buf = vim.api.nvim_create_buf(true, true) -- No file, scratch buffer
    vim.bo[results_buf].bufhidden = 'wipe'
    vim.bo[results_buf].modifiable = true

    -- Set the lines of the buffer
    vim.api.nvim_buf_set_lines(results_buf, 0, -1, false, lines)

    -- Optionally, open the buffer in a new window
    vim.api.nvim_win_set_buf(0, results_buf)

    -- Set a keymap for pressing <CR> on a line to open the URL.
    -- Use a Lua callback for easy parsing.
    vim.keymap.set('n', '<CR>', function()
        local line = vim.api.nvim_get_current_line()
        -- Assume the format "Title - URL"
        local url = line:match(" %- (.+)$") -- captures everything after " - "
        local req = { url = url }
        local res = curl.get(req)
        if res.status == 200 then
            local res_lines = {}
            lines = {}
            for s in res.body:gmatch("[^\r\n]+") do
                table.insert(res_lines, s)
            end
            vim.api.nvim_buf_set_name(results_buf, "RFSee result")
            vim.api.nvim_buf_set_lines(results_buf, 0, -1, false, res_lines)
            vim.keymap.del('n', '<CR>', { buffer = results_buf })
        end
    end, { buffer = results_buf, noremap = true, silent = true })
end

-- Our Lua callback, cast to a C function pointer

function M.refresh()
    local start_time = os.clock()
    local buf, win = window.create_progress_window()
    window.update_progress_window(buf, "Building RFC index")

    -- local function progress_cb(ptr)
    --     -- local msg = ffi.string(ptr)
    --     -- -- local msg = string.format("Downloading RFCs progress: %.1f%%", pct)
    --     -- window.update_progress_window(buf, msg)
    --
    --     local ok, err = xpcall(function()
    --         print("Getting string")
    --         io.stdout:flush()
    --         local msg = ffi.string(ptr) -- May throw if `ptr` is invalid
    --         print("Got string")
    --         io.stdout:flush()
    --         window.update_progress_window(buf, msg)
    --     end, debug.traceback)
    --     print("Ok: ", ok)
    --     io.stdout:flush()
    --     if not ok then
    --         -- Log the error, but don't let it unwind into Rust
    --         print("Error in progress_cb:", err)
    --         vim.cmd("redraw")
    --     end
    -- end

    local function real_progress_cb(ptr)
        print("Inside real_progress_cb, about to ffi.string(ptr)")
        io.stdout:flush()

        local msg = ffi.string(ptr) -- If this fails, it won't kill the process if wrapped by pcall
        print("Successfully got msg:", msg)
        io.stdout:flush()

        window.update_progress_window(buf, msg)
    end

    local function safe_progress_cb(ptr)
        local ok, err = xpcall(function()
            real_progress_cb(ptr)
        end, debug.traceback)

        if not ok then
            print("Lua callback error:", err)
            io.stdout:flush()
            -- DO NOT re-throw the error; otherwise it unwinds back into Rust
        end
    end

    local progress_cb_c = ffi.cast("progress_callback_t", safe_progress_cb)

    -- M.progress_cb_c = ffi.cast("progress_callback_t", progress_cb)
    lib.build_index(progress_cb_c)
    local end_time = os.clock()
    window.update_progress_window(buf, string.format("Built RFC index", end_time - start_time))
    -- Brief pause before closing
    os.execute("sleep 1")
    window.close_progress_window(win)
end

return M
