local M = {}

---@param input string The input string
---@param sep string The string to split on
---@return string[] The list of elements after splitting on `sep`
function M.split(input, sep)
    if sep == nil or sep == "" then
        sep = "%s" -- Default to splitting on whitespace
    end

    local t = {}
    local pattern = "(.-)" .. sep

    for match, _ in string.gmatch(input .. sep, pattern) do
        table.insert(t, match)
    end

    return t
end

---@param s string The input string
---@param pattern string|number The match pattern
function M.enumerate_gmatch(s, pattern)
    local counter = 0
    local iterator = string.gmatch(s, pattern)
    return function()
        counter = counter + 1
        local match = iterator()
        if match then
            return counter, match
        else
            return nil
        end
    end
end

return M
