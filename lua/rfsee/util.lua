local M = {}

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

-- function M.map(array, func)
--     local new_array = {}
--     for i, value in ipairs(array) do
--         new_array[i] = func(value)
--     end
--     return new_array
-- end

return M
