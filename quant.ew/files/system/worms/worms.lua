local worms = {}

local function get_closest_alive(x, y)
    local min_dist
    local min_ent
    for _, player in pairs(ctx.players) do
        if player.status.is_alive then
            local tx, ty = EntityGetTransform(player.entity)
            local dx, dy = tx - x, ty - y
            local dist = dx * dx + dy * dy
            if min_dist == nil or dist < min_dist then
                min_dist = dist
                min_ent = player.entity
            end
        end
    end
    return min_ent
end

function worms.on_world_update()
    if GameGetFrameNum() % 10 ~= 0 then
        return
    end
    for _, ent in ipairs(EntityGetWithTag("boss_dragon") or {}) do
        local dragon = EntityGetFirstComponentIncludingDisabled(ent, "BossDragonComponent")
        local x, y = EntityGetTransform(ent)
        local min_ent = get_closest_alive(x, y)
        if min_ent ~= nil then
            ComponentSetValue2(dragon, "mTargetEntityId", min_ent)
        end
    end
    for _, ent in ipairs(EntityGetWithTag("worm") or {}) do
        local worm = EntityGetFirstComponentIncludingDisabled(ent, "WormAIComponent")
        if EntityHasTag(ComponentGetValue2(worm, "mTargetEntityId"), "ew_notplayer") then
            local x, y = EntityGetTransform(ent)
            local min_ent = get_closest_alive(x, y)
            if min_ent ~= nil then
                ComponentSetValue2(worm, "mTargetEntityId", min_ent)
            end
        end
    end
end

return worms