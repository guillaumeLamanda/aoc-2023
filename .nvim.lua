---@param day string
---@param cb function
local function ensure_scaffolding(day, cb)
    local source_file = string.format("src/bin/%02d.rs", day)
    if vim.fn.filereadable(source_file) == 0 then
        vim.loop.spawn("cargo", { args = { "scaffold", day } }, function(code)
            if code ~= 0 then
                print("Error: " .. code)
            else
                vim.notify("scaffolded day " .. day)
                cb()
            end
        end)
    else
        cb()
    end
end

---@param day string
---@param cb function
local function ensure_download(day, cb)
    local input_file = string.format("data/inputs/%02d.txt", day)
    local should_domnload = vim.fn.filereadable(input_file) == 0 or io.read("*n") == nil
    if should_domnload then
        vim.notify("downloading day " .. day)
        vim.loop.spawn("cargo", { args = { "download", day } }, function(code)
            if code ~= 0 then
                print("Error: " .. code)
            else
                cb()
            end
        end)
    else
        cb()
    end
end

local function open_files(day)
    local instructions_file = string.format("data/puzzles/%02d.md", day)
    local input_file = string.format("data/inputs/%02d.txt", day)
    local source_file = string.format("src/bin/%02d.rs", day)

    vim.cmd("e " .. source_file)
    vim.cmd("vsplit " .. instructions_file)
    vim.cmd("split " .. input_file)
end

local function open_aoc()
    local current_day = os.date("*t").day
    local day = vim.fn.input({ prompt = "Day: ", default = current_day })
    if not day or day == "" then
        return
    end

    ensure_scaffolding(
        day,
        vim.schedule_wrap(function()
            ensure_download(
                day,
                vim.schedule_wrap(function()
                    open_files(day)
                end)
            )
        end)
    )

    -- open thoses files split vertically
end

vim.keymap.set("n", "<leader>po", open_aoc, { desc = "Open advent of code files" })
vim.keymap.set("n", "<leader>pc", function()
    vim.cmd("bdelete src/bin/*.rs data/input/*.txt data/puzzles/*.md")
end, { desc = "Close advent of code files" })
