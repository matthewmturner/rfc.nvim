local log = require("plenary.log").new {
    plugin = "rfsee",                               -- Your plugin's name
    level = "info",                                 -- Log level: "trace", "debug", "info", "warn", "error"
    use_console = true,                             -- Show logs in the command line (optional)
    use_file = true,                                -- Save logs to a file
    file = vim.fn.stdpath("cache") .. "/rfsee.log", -- Log file path
}

return log
