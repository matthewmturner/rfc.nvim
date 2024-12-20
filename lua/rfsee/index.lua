local plenary = require('plenary')
local log = require("rfsee.log")
local parse = require("rfsee.parse")
local tf_idf = require("rfsee.tf_idf")
local r = require("rfsee.ffi")
local ffi = require("ffi")

local M = {}

RFC_INDEX_URL = "https://www.ietf.org/rfc/rfc-index.txt"
RFC_DELIMITTER = "\n\n"

function M.search_terms(terms)
    local results = r.search_terms(terms)

    -- print("Got results")
    -- Check for errors
    if results == nil or results.error ~= 0 then
        print("Error occurred during search")
        return
    end
    -- print("no errors, count", results.len)

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
    -- vim.api.nvim_command('vsplit')
    vim.api.nvim_win_set_buf(0, buf)

    -- Set a keymap for pressing <CR> on a line to open the URL.
    -- Use a Lua callback for easy parsing.
    vim.keymap.set('n', '<CR>', function()
        local line = vim.api.nvim_get_current_line()
        -- Assume the format "Title - URL"
        local url = line:match(" %- (.+)$") -- captures everything after " - "
        print("Chose URL: ", url)
        if url then
            -- Open the URL in the default browser (Linux/FreeBSD with xdg-open)
            -- For macOS, use `open`. For Windows, use `start`.
            vim.system({ 'open', url }):wait()
            -- vim.fn.jobstart({ 'open', url }, { detach = true })
        else
            print("No URL found on line!")
        end
    end, { buffer = buf, noremap = true, silent = true })

    -- if results.error == 0 then
    --     for i = 0, results.len - 1 do
    --         local rfc = results.rfcs[i]
    --         print("URL:", ffi.string(rfc.url), "Title:", ffi.string(rfc.title))
    --     end
    -- end
end

function M.refresh()
    r.build_index()
end

return M
