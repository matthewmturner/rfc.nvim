local util = require("rfsee.util")

---@alias RFC { number: number, status: string }
---@alias RFCStatus 'UNKNOWN' | 'INFORMATIONAL' | 'EXPERIMENTAL' | 'HISTORIC' | 'DRAFT STANDARD' | 'PROPOSED STANDARD' | 'INTERNET STANDARD'

STATUS_MATCHER = "Status:"

---@param content string RFC string contents
---@return RFCStatus|nil The RFCs status
local function parse_rfc_status(content)
    print("Content: ", content)
    local found_status = string.find(content, STATUS_MATCHER)
    if found_status then
        local splitted = util.split(content, STATUS_MATCHER)
        local status = splitted[2]
        return status
    end
end

local M = {}

-- Extraft the RFCs from index response body
---@param body string Response body
---@return table<string>|nil splitted table of extracted RFC strings, or `nil` if no RFCs are found.
function M.parse_rfcs(body)
    local found = string.find(body, "0001")
    if found then
        local raw_rfcs = string.sub(body, found)
        local splitted = util.split(raw_rfcs, RFC_DELIMITTER)
        return splitted
    end
end

-- Parse RFC index entry string into its components
---@param entry string RFC index entry
---@return RFC|nil rfc Parsed RFC
function M.parse_rfc(entry)
    local after_rfc_num = string.find(entry, " ")
    if not after_rfc_num then
        return
    end
    local rfc = {}
    rfc.number = string.sub(entry, 0, after_rfc_num)
    local content = string.sub(entry, after_rfc_num)
    for matched in string.gmatch(content, "%((.-)%)") do
        local status = parse_rfc_status(matched)
        print("Status: ", status)
        if type(status) == "string" then
            print("Status: ", status)
        end
        -- if not status == nil then
        --     print("Status: ", status)
        -- end
        -- local has_text = parse_rfc_has_txt(matched)
        -- print(matched)
    end
    return rfc
end

return M
