local M = {}

local plenary = require('plenary')

-- function Split(input, sep)
--     if sep == nil then
--         sep = "%s"
--     end
--     local t = {}
--     for str in string.gmatch(input, "([^" .. sep .. "]+)") do
--         table.insert(t, str)
--     end
--     return t
-- end

function Split(input, sep)
    if sep == nil or sep == "" then
        sep = "%s" -- Default to splitting on whitespace
    end

    local t = {}
    local pattern = "(.-)" .. sep
    local last_pos = 1

    for match, pos in string.gmatch(input .. sep, pattern) do
        table.insert(t, match)
        last_pos = pos
    end

    return t
end

function IndexRFC(opts)
    print("Indexing RFCs: ", opts)
    local params = {
        url = "https://www.ietf.org/rfc/rfc-index.txt"
    }
    local res = plenary.curl.get(params)
    if res.status == 200 then
        ---@type string
        local body = res.body
        local found = string.find(body, "0001")
        if found then
            print("Found: ", found)
            local rfcs_content = string.sub(body, found)
            local rfcs = Split(rfcs_content, "\n\n")
            print("First RFC: ", rfcs[1])
            print("Second RFC: ", rfcs[2])
        end
    end
end

function M.setup(opts)
    -- print("Setup in init: %s", opts)
    vim.api.nvim_create_user_command("RFCIndex", IndexRFC, {})
end

return M
