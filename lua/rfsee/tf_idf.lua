local plenary = require('plenary')
local window = require("rfsee.window")
local lib = require("rfsee.ffi")

-- rfsee.parse.lua

---@alias TermFreqs {}
---@alias TfIdfIndex {}


-- https://en.wikipedia.org/wiki/Tf%E2%80%93idf#Term-frequency
---@param rfc_text string The raw text content of the RFC
---@return TermFreqs frequences The term frequencies of input text
local function extract_term_frequencies(rfc_text)
    -- TODO: Move to Rust / FFI
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

    local tf = lib.tf_create()
    for t, c in pairs(token_counts) do
        lib.tf_insert_term(tf, t, c / terms)
    end

    return tf
end

local M = {}

RFC_URL_BASE = "https://www.rfc-editor.org/rfc/rfc"
RFC_URL_SUFFIX = ".txt"

---@param rfcs RFC[] Parsed RFCs
---@return TfIdfIndex index Built index from parsed RFCs
function M.build_index(rfcs)
    local index = lib.tf_idf_create()
    local buf, win = window.create_progress_window()
    for i, rfc in pairs(rfcs) do
        local url = string.format("%s%s%s", RFC_URL_BASE, rfc.number, RFC_URL_SUFFIX)
        local params = {
            url = url
        }
        if i % 100 == 0 then
            local msg = string.format("Processed RFC %s", i)
            window.update_progress_window(buf, msg)
            vim.cmd("redraw")
        end
        local rfc_res = plenary.curl.get(params)
        if rfc_res.status == 200 then
            local tf = extract_term_frequencies(rfc_res.body)
            lib.tf_idf_insert_doc_tfs(index, url, tf)
        else
            print(rfc_res.status)
        end
    end

    window.update_progress_window(buf, "Finishing index")
    lib.tf_idf_finish(index)

    window.update_progress_window(buf, "Saving index")
    lib.tf_idf_save(index, "./index.json")
    window.close_progress_window(win)
    return index
end

return M
