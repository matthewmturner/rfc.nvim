local plenary = require('plenary')
local util = require("rfsee.util")
local log = require("rfsee.log")
local parse = require("rfsee.parse")

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
    local rfc_index_body = get_raw_index()
    if type(rfc_index_body) == "string" then
        local rfcs = parse.parse_rfcs(rfc_index_body)
        if rfcs then
            for _, entry in ipairs(rfcs) do
                local rfc = parse.parse_rfc(entry)
            end
        end
    end
end

return M
