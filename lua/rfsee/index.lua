local lib      = require("rfsee.ffi")
local window   = require("rfsee.window")
local ffi      = require("ffi")

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
    local buf = vim.api.nvim_create_buf(false, true) -- No file, scratch buffer
    vim.bo[buf].bufhidden = 'wipe'
    vim.bo[buf].modifiable = true

    -- Set the lines of the buffer
    vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)

    -- Optionally, open the buffer in a new window
    vim.api.nvim_win_set_buf(0, buf)

    -- Set a keymap for pressing <CR> on a line to open the URL.
    -- Use a Lua callback for easy parsing.
    vim.keymap.set('n', '<CR>', function()
        local line = vim.api.nvim_get_current_line()
        -- Assume the format "Title - URL"
        local url = line:match(" %- (.+)$") -- captures everything after " - "
        if url then
            -- Open the URL in the default browser (Linux/FreeBSD with xdg-open)
            -- For macOS, use `open`. For Windows, use `start`.
            vim.system({ 'open', url }):wait()
        else
            print("No URL found on line!")
        end
    end, { buffer = buf, noremap = true, silent = true })
end

function M.refresh()
    local start_time = os.clock()
    local buf, win = window.create_progress_window()
    window.update_progress_window(buf, "Building RFC index")
    lib.build_index()
    local end_time = os.clock()
    window.update_progress_window(buf, string.format("Built RFC index in %.2f seconds", end_time - start_time))
    os.execute("sleep 30")
    window.close_progress_window(win)
end

return M
