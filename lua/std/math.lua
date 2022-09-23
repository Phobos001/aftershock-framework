-- Extra math functions you can use in your game!


function lerp(a, b, t)
    return a + (b - a) * t
end

-- 2D Vectors
function distance(x0, y0, x1, y1)
    return math.sqrt((x1 - x0) ^ 2.0 + (y1 - y0) ^ 2.0)
end

function magnitude(x, y)
    return math.sqrt((x * x) + (y * y))
end

function normalize(x, y)
    local magnitude = magnitude(x, y)
    return x / magnitude, y / magnitude
end