local M = {}

function M.create_progress_window()
    local buf = vim.api.nvim_create_buf(false, true)
    local width = 20
    local height = 1
    local row = 0
    local col = vim.o.columns - (width + 1) -- Place it at the top right
    local win = vim.api.nvim_open_win(buf, false, {
        relative = "editor",
        width = width,
        height = height,
        row = row,
        col = col,
        style = "minimal",
        border = "rounded",
    })
    return buf, win
end

function M.update_progress_window(buf, message)
    vim.api.nvim_buf_set_lines(buf, 0, -1, false, { message })
    vim.cmd("redraw")
end

function M.close_progress_window(win)
    vim.api.nvim_win_close(win, true)
    vim.cmd("redraw")
end

return M
