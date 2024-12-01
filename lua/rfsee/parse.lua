local util = require("rfsee.util")

---@alias RFC { number: number, title: string, status: string, formats: string[] }
---@alias RFCStatus 'UNKNOWN' | 'INFORMATIONAL' | 'EXPERIMENTAL' | 'HISTORIC' | 'DRAFT STANDARD' | 'PROPOSED STANDARD' | 'INTERNET STANDARD'

STATUS_MATCHER = "Status:"
FORMATS_MATCHER = "Format:"

---@param content string RFC string contents
---@return RFCStatus|nil The RFCs status
local function parse_rfc_status(content)
    local content_without_newlines = string.gsub(content, "\n", "")
    local found_status = string.find(content_without_newlines, STATUS_MATCHER)
    if found_status then
        local splitted = util.split(content_without_newlines, STATUS_MATCHER)
        local status = splitted[2]
        return status
    end
end

---@param content string RFC string contents
---@return string[]|nil The RFCs formats
local function parse_rfc_formats(content)
    local content_without_newlines = string.gsub(content, "\n", "")
    local found_status = string.find(content_without_newlines, FORMATS_MATCHER)
    if found_status then
        local splitted = util.split(content_without_newlines, FORMATS_MATCHER)
        local formats_string = splitted[2]
        local splitted_formats = util.split(formats_string, ",")
        local cleaned_formats = {}
        for i, f in ipairs(splitted_formats) do
            cleaned_formats[i] = string.gsub(f, ' ', '')
        end
        return cleaned_formats
    end
end

---@param formats string[] RFC string contents
---@return boolean The RFCs status
local function has_txt_format(formats)
    for _, f in ipairs(formats) do
        if f == "TXT" then
            return true
        end
    end
    return false
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
    local title_end = string.find(entry, "%(")
    local title = string.sub(string.sub(entry, 0, title_end), 0, -3)
    local content = string.sub(entry, after_rfc_num)
    local rfc_number = string.sub(string.sub(entry, 0, after_rfc_num), 0, -2)
    status = nil
    formats = nil
    for matched in string.gmatch(content, "%((.-)%)") do
        rfc.title = title
        rfc.number = tonumber(rfc_number)
        local status = parse_rfc_status(matched)
        if status ~= nil then
            rfc.status = status
        end
        local formats = parse_rfc_formats(matched)
        if formats ~= nil then
            rfc.formats = formats
        end
    end
    if type(rfc.title) == "string" and type(rfc.number) == "number" and type(rfc.status) == "string" and rfc.formats ~= nil and has_txt_format(rfc.formats) then
        return rfc
    end
    return nil
end

return M
