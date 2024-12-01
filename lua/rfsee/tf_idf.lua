local plenary = require('plenary')
local parse = require("rfsee.parse")
local util = require("rfsee.util")

-- rfsee.parse.lua

---@alias TfIdfIndex {}

---@param rfc_text string The raw text content of the RFC
---@return string[] tokens Parsed tokens
local function tokenize(rfc_text)
    local splitted = util.split(rfc_text, " ")
    for _, item in ipairs(splitted) do
    end
end

---@param rfc_text string The raw text content of the RFC
local function extract_term_frequencies(rfc_text)
    local tokens = tokenize(rfc_text)
    local frequencies = {}
    for _, token in ipairs(tokens) do
        if frequencies[token] ~= nil then
            frequencies[token] = 1
        else
            frequencies[token] = frequencies[token] + 1
        end
    end
end

local M = {}

RFC_URL_BASE = "https://www.rfc-editor.org/rfc/rfc"
RFC_URL_SUFFIX = ".txt"

---@param rfcs RFC[] Parsed RFCs
---@return TfIdfIndex index Built index from parsed RFCs
function M.build_index(rfcs)
    local index = {}
    for _, rfc in ipairs(rfcs) do
        local params = {
            url = string.format("%s%s%s", RFC_URL_BASE, rfc.number, RFC_URL_SUFFIX)
        }
        local rfc_res = plenary.curl.get(params)
        if rfc_res.status == 200 then
            local term_frequencies = extract_term_frequencies(rfc_res.body)
        else
            print(rfc_res.status)
        end
    end
    return index
end

return M
