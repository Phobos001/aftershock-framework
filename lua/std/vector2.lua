-- Vector2 Implementation
-- Functional, no closures or tables

function vec2_distance(x0, y0, x1, y1)
    return math.sqrt((x1 - x0) ^ 2.0 + (y1 - y0) ^ 2.0)
end

function vec2_magnitude(x, y)
    return math.sqrt((x * x) + (y * y))
end

function vec2_dot(x0, y0, x1, y1)
    return x0 * x1 + y0 * y1
end

function vec2_normalize(x, y)
    local magnitude = math.sqrt((x * x) + (y * y))
    return x / magnitude, y / magnitude
end

function vec2_lerp(x0, y0, x1, y1, t)
    local x = x0 + (x1 - x0) * t
    local y = y0 + (y1 - y0) * t
    return x, y
end