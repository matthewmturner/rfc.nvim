local plenary = require('plenary')
local log = require("rfsee.log")
local parse = require("rfsee.parse")
local tf_idf = require("rfsee.tf_idf")
local r = require("rfsee.rust")

local M = {}

RFC_INDEX_URL = "https://www.ietf.org/rfc/rfc-index.txt"
RFC_DELIMITTER = "\n\n"

-- Makes HTTP GET request for RFC index and if request is successful
-- return the body of the response.
local function get_raw_index()
    local params = {
        url = RFC_INDEX_URL
    }
    local res = plenary.curl.get(params)

    if res.status == 200 then
        if type(res.body) == "string" then
            local body = res.body --[[@as string]]
            return body
        end

        log.debug("Response body is not a string")
    end
    return nil
end


function M.refresh()
    local save_result = r.save_input_number_as_json_to_custom_path(7, "/tmp/my_path.json")
    print("Save result error: ", save_result.error)
    print("Cache dir", vim.fn.stdpath("cache"))
    local rfc_index_body = get_raw_index()
    if type(rfc_index_body) == "string" then
        local raw_rfcs = parse.parse_rfcs(rfc_index_body)
        local rfcs = {}
        if raw_rfcs then
            for i, entry in ipairs(raw_rfcs) do
                rfcs[i] = parse.parse_rfc(entry)
            end
        end

        local index = tf_idf.build_index(rfcs)
    end
end

return M
