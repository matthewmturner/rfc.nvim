local plenary = require('plenary')
local parse = require("rfsee.parse")

-- rfsee.parse.lua

---@alias TfIdfIndex {}

local M = {}

RFC_URL_BASE = "https://www.rfc-editor.org/rfc/rfc"

---@param rfcs RFC[] Parsed RFCs
---@return TfIdfIndex index Built index from parsed RFCs
function M.build_index(rfcs)
    local index = {}
    for _, rfc in ipairs(rfcs) do
        local params = {
            url = string.format("%s%s%s", RFC_URL_BASE, rfc.number, ".txt")
        }
        print("Getting RFC: ", params.url)
        local rfc_res = plenary.curl.get(params)
        if rfc_res.status == 200 then
            print(rfc_res.body)
        else
            print(rfc_res.status)
        end
    end
    return index
end

return M
