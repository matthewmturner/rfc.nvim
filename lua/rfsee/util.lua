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

return M
