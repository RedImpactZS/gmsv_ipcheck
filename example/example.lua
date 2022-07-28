require("ipcheck")

ipcheck.load(file.Read("./french.txt","LUA"))

hook.Add("PlayerConnect","frenchDetector.PlayerConnect", function( name, ip )
    -- you can cache result if needed
    if ipcheck.contains(ip:match("^(.+):")) then
        -- issue french player alert
    end
end)