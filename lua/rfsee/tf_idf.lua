local plenary = require('plenary')
local parse = require("rfsee.parse")
local util = require("rfsee.util")
local r = require("rfsee.rust")

-- rfsee.parse.lua

---@alias TfIdfIndex {}

-- https://en.wikipedia.org/wiki/Tf%E2%80%93idf#Term-frequency
---@param rfc_text string The raw text content of the RFC
---@return table<string, float>, unknown frequences The term frequencies of input text
local function extract_term_frequencies(rfc_text)
    local token_counts = {}
    local terms = 0
    for token in string.gmatch(rfc_text, "%a+") do
        local lower_case_token = string.lower(token)
        if token_counts[lower_case_token] == nil then
            token_counts[lower_case_token] = 1
        else
            token_counts[lower_case_token] = token_counts[lower_case_token] + 1
        end
        terms = terms + 1
    end

    local frequencies = {}
    local tf = r.create_term_freqs()
    for t, c in pairs(token_counts) do
        r.insert_term_freqs(tf, t, c / terms)
        frequencies[t] = c / terms
    end

    return frequencies, tf
end

local M = {}

RFC_URL_BASE = "https://www.rfc-editor.org/rfc/rfc"
RFC_URL_SUFFIX = ".txt"

---@param rfcs RFC[] Parsed RFCs
---@return TfIdfIndex index Built index from parsed RFCs
function M.build_index(rfcs)
    local index = {}
    local ffi_index = r.create_tf_idf()
    for _, rfc in ipairs(rfcs) do
        local url = string.format("%s%s%s", RFC_URL_BASE, rfc.number, RFC_URL_SUFFIX)
        local params = {
            url = url
        }
        local rfc_res = plenary.curl.get(params)
        if rfc_res.status == 200 then
            local term_frequencies, ffi_tf = extract_term_frequencies(rfc_res.body)
            index[url] = term_frequencies
            r.insert_tf_idf(ffi_index, url, ffi_tf)
            -- for t, f in pairs(term_frequencies) do
            --     print(t, f)
            -- end
        else
            print(rfc_res.status)
        end
    end
    r.save_tf_idf(ffi_index, "./index.json")
    return index
end

return M
