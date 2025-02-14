local M = {}

local index = require("rfsee.index")

---@diagnostic disable-next-line: unused-local
function M.setup(opts)
    vim.api.nvim_create_user_command("RFSeeIndex", index.refresh, {})
    vim.api.nvim_create_user_command(
        "RFSee",
        function(opts)
            index.search_terms(opts.args)
        end,
        {
            nargs = 1
        })
end

return M
